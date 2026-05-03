use diesel::prelude::*;
use uuid::Uuid;

use crate::application::{
    common::repo_error::RepoError,
    group::{
        dto::{
            GroupDetails, GroupFromUserDetails, GroupMemberDetails, HistoricGroupMemberDetails,
            UserInGroupDetails,
        },
        traits::repository::GroupRepository,
    },
};

use crate::domain::group::{Group, GroupConfig, GroupId, GroupMember};
use crate::domain::user::UserId;

use crate::infrastructure::db::{
    models::group::{
        GroupMemberStatusModel, GroupModel, GroupRoleModel, GroupStatusModel, NewGroupModel,
        NewUserInGroupModel, UserInGroupModel,
    },
    models::user::UserModel,
    pool::{DbConn, DbPool},
    schema,
};

pub struct DieselGroupRepository {
    db: DbPool,
}

impl DieselGroupRepository {
    pub fn new(db: DbPool) -> Self {
        Self { db }
    }

    fn get_conn(&self) -> Result<DbConn, RepoError> {
        self.db.get().map_err(|_| RepoError::Connection)
    }

    fn load_active_members(
        conn: &mut DbConn,
        group_id: Uuid,
    ) -> Result<Vec<GroupMember>, RepoError> {
        let rows = schema::user_in_group::table
            .filter(schema::user_in_group::group_id.eq(group_id))
            .filter(schema::user_in_group::status.eq(GroupMemberStatusModel::Active))
            .select(UserInGroupModel::as_select())
            .get_results::<UserInGroupModel>(conn)
            .map_err(|_| RepoError::Query)?;

        Ok(rows.into_iter().map(GroupMember::from).collect())
    }

    fn assemble_group(conn: &mut DbConn, group_model: GroupModel) -> Result<Group, RepoError> {
        let members = Self::load_active_members(conn, group_model.id)?;

        Ok(Group::rehydrate(
            GroupId(group_model.id),
            group_model.name,
            group_model.description,
            matches!(group_model.status, GroupStatusModel::Active),
            GroupConfig::default(),
            members,
        ))
    }
}

impl GroupRepository for DieselGroupRepository {
    fn find_by_id(&self, id: GroupId) -> Result<Option<Group>, RepoError> {
        let mut conn = self.get_conn()?;

        let group_model = schema::group::table
            .filter(schema::group::id.eq(id.0))
            .select(GroupModel::as_select())
            .first::<GroupModel>(&mut conn)
            .optional()
            .map_err(|_| RepoError::Query)?;

        match group_model {
            Some(g) => Ok(Some(Self::assemble_group(&mut conn, g)?)),
            None => Ok(None),
        }
    }

    fn save(&self, group: &Group) -> Result<(), RepoError> {
        let mut conn = self.get_conn()?;

        conn.transaction::<(), diesel::result::Error, _>(|conn| {
            let status = if group.is_active {
                GroupStatusModel::Active
            } else {
                GroupStatusModel::Ended
            };

            let new_group = NewGroupModel {
                id: group.id.0,
                name: group.name.clone(),
                description: group.description.clone(),
            };

            diesel::insert_into(schema::group::table)
                .values(&new_group)
                .on_conflict(schema::group::id)
                .do_update()
                .set((
                    schema::group::name.eq(&group.name),
                    schema::group::description.eq(&group.description),
                    schema::group::status.eq(status),
                    schema::group::updated_at.eq(chrono::Utc::now().naive_utc().date()),
                ))
                .execute(conn)?;

            let existing: Vec<UserInGroupModel> = schema::user_in_group::table
                .filter(schema::user_in_group::group_id.eq(group.id.0))
                .select(UserInGroupModel::as_select())
                .get_results(conn)?;

            for member in &group.members {
                let role: GroupRoleModel = member.role.into();
                let already = existing.iter().any(|e| e.user_id == member.user_id.0);

                if already {
                    diesel::update(schema::user_in_group::table)
                        .filter(schema::user_in_group::user_id.eq(member.user_id.0))
                        .filter(schema::user_in_group::group_id.eq(group.id.0))
                        .set((
                            schema::user_in_group::role.eq(role),
                            schema::user_in_group::status.eq(GroupMemberStatusModel::Active),
                            schema::user_in_group::updated_at.eq(chrono::Utc::now().naive_utc()),
                        ))
                        .execute(conn)?;
                } else {
                    diesel::insert_into(schema::user_in_group::table)
                        .values(&NewUserInGroupModel {
                            user_id: member.user_id.0,
                            group_id: group.id.0,
                            role,
                        })
                        .execute(conn)?;
                }
            }

            for existing_row in &existing {
                let still_present = group
                    .members
                    .iter()
                    .any(|m| m.user_id.0 == existing_row.user_id);

                if !still_present && existing_row.status == GroupMemberStatusModel::Active {
                    diesel::update(schema::user_in_group::table)
                        .filter(schema::user_in_group::user_id.eq(existing_row.user_id))
                        .filter(schema::user_in_group::group_id.eq(group.id.0))
                        .set((
                            schema::user_in_group::status.eq(GroupMemberStatusModel::Left),
                            schema::user_in_group::updated_at.eq(chrono::Utc::now().naive_utc()),
                        ))
                        .execute(conn)?;
                }
            }

            Ok(())
        })
        .map_err(|_| RepoError::Insert)?;

        Ok(())
    }

    fn find_by_user(&self, user_id: UserId) -> Result<Vec<Group>, RepoError> {
        let mut conn = self.get_conn()?;

        let group_models: Vec<GroupModel> = schema::group::table
            .inner_join(schema::user_in_group::table)
            .filter(schema::user_in_group::user_id.eq(user_id.0))
            .filter(schema::user_in_group::status.eq(GroupMemberStatusModel::Active))
            .select(GroupModel::as_select())
            .get_results(&mut conn)
            .map_err(|_| RepoError::Query)?;

        let mut groups = Vec::with_capacity(group_models.len());
        for g in group_models {
            groups.push(Self::assemble_group(&mut conn, g)?);
        }

        Ok(groups)
    }

    fn get_group_details(&self, id: GroupId) -> Result<Option<GroupDetails>, RepoError> {
        let mut conn = self.get_conn()?;
        let group_model = schema::group::table
            .filter(schema::group::id.eq(id.0))
            .select(GroupModel::as_select())
            .first::<GroupModel>(&mut conn)
            .optional()
            .map_err(|_| RepoError::Query)?;

        Ok(group_model.map(|g| GroupDetails {
            id: g.id,
            name: g.name,
            description: g.description,
            status: g.status,
            created_at: g.created_at,
            updated_at: g.updated_at,
        }))
    }

    fn get_group_members(&self, group_id: GroupId) -> Result<Vec<GroupMemberDetails>, RepoError> {
        let mut conn = self.get_conn()?;
        let rows = schema::user_in_group::table
            .inner_join(schema::user::table)
            .filter(schema::user_in_group::group_id.eq(group_id.0))
            .filter(schema::user_in_group::status.eq(GroupMemberStatusModel::Active))
            .select((UserInGroupModel::as_select(), UserModel::as_select()))
            .get_results::<(UserInGroupModel, UserModel)>(&mut conn)
            .map_err(|_| RepoError::Query)?;

        Ok(rows
            .into_iter()
            .map(|(rel, user)| GroupMemberDetails {
                user_id: user.id,
                group_id: rel.group_id,
                name: user.name,
                email: user.email,
                status: rel.status,
                role: rel.role,
            })
            .collect())
    }

    fn get_historic_group_members(
        &self,
        group_id: GroupId,
    ) -> Result<Vec<HistoricGroupMemberDetails>, RepoError> {
        let mut conn = self.get_conn()?;
        let rows = schema::user_in_group::table
            .inner_join(schema::user::table)
            .filter(schema::user_in_group::group_id.eq(group_id.0))
            .select((UserInGroupModel::as_select(), UserModel::as_select()))
            .get_results::<(UserInGroupModel, UserModel)>(&mut conn)
            .map_err(|_| RepoError::Query)?;

        Ok(rows
            .into_iter()
            .map(|(rel, user)| HistoricGroupMemberDetails {
                user_id: user.id,
                group_id: rel.group_id,
                name: user.name,
                email: user.email,
                status: rel.status,
                role: rel.role,
            })
            .collect())
    }

    fn get_user_groups_legacy(
        &self,
        user_id: UserId,
    ) -> Result<Vec<GroupFromUserDetails>, RepoError> {
        let mut conn = self.get_conn()?;
        let rows = schema::group::table
            .inner_join(schema::user_in_group::table)
            .filter(schema::user_in_group::user_id.eq(user_id.0))
            .filter(schema::user_in_group::status.eq(GroupMemberStatusModel::Active))
            .select((GroupModel::as_select(), UserInGroupModel::as_select()))
            .get_results::<(GroupModel, UserInGroupModel)>(&mut conn)
            .map_err(|_| RepoError::Query)?;

        Ok(rows
            .into_iter()
            .map(|(group, relation)| GroupFromUserDetails {
                user_id: relation.user_id,
                group_id: group.id,
                role: relation.role,
                group_name: group.name,
                group_description: group.description,
                status: group.status,
            })
            .collect())
    }

    fn get_user_in_group(
        &self,
        user_id: UserId,
        group_id: GroupId,
    ) -> Result<Option<UserInGroupDetails>, RepoError> {
        let mut conn = self.get_conn()?;
        let row = schema::user_in_group::table
            .filter(schema::user_in_group::user_id.eq(user_id.0))
            .filter(schema::user_in_group::group_id.eq(group_id.0))
            .select(UserInGroupModel::as_select())
            .first::<UserInGroupModel>(&mut conn)
            .optional()
            .map_err(|_| RepoError::Query)?;
        Ok(row.map(|rel| UserInGroupDetails {
            user_id: rel.user_id,
            group_id: rel.group_id,
            role: rel.role,
            status: rel.status,
            joined_at: rel.joined_at,
            updated_at: rel.updated_at,
        }))
    }

    fn is_member(&self, user_id: UserId, group_id: GroupId) -> Result<bool, RepoError> {
        let mut conn = self.get_conn()?;
        let row = schema::user_in_group::table
            .filter(schema::user_in_group::group_id.eq(group_id.0))
            .filter(schema::user_in_group::user_id.eq(user_id.0))
            .filter(schema::user_in_group::status.eq(GroupMemberStatusModel::Active))
            .select(UserInGroupModel::as_select())
            .first::<UserInGroupModel>(&mut conn)
            .optional()
            .map_err(|_| RepoError::Query)?;
        Ok(row.is_some())
    }

    fn is_admin(&self, user_id: UserId, group_id: GroupId) -> Result<bool, RepoError> {
        let mut conn = self.get_conn()?;
        let row = schema::user_in_group::table
            .filter(schema::user_in_group::group_id.eq(group_id.0))
            .filter(schema::user_in_group::user_id.eq(user_id.0))
            .filter(schema::user_in_group::role.eq(GroupRoleModel::Admin))
            .filter(schema::user_in_group::status.eq(GroupMemberStatusModel::Active))
            .select(UserInGroupModel::as_select())
            .first::<UserInGroupModel>(&mut conn)
            .optional()
            .map_err(|_| RepoError::Query)?;
        Ok(row.is_some())
    }
}
