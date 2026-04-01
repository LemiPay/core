use crate::data::database::Db;
use crate::data::error::DbError;
use crate::models::group::{Group, GroupUpdate, MyGroupStatus, NewGroup};
use crate::models::user::User;
use crate::models::user_in_group::{
    GroupFromUser, GroupMember, MyGroupMemberStatus, MyGroupRole, NewUserInGroup, UserInGroup,
};
use crate::repositories::traits::group_repo::GroupRepository;
use crate::schema::group;
use crate::schema::user;
use crate::schema::user_in_group;
use diesel::prelude::*;
use uuid::Uuid;

pub struct DieselGroupRepository {
    db: Db,
}

impl DieselGroupRepository {
    pub fn new(db: Db) -> Self {
        Self { db }
    }
}

impl GroupRepository for DieselGroupRepository {
    fn create_group(
        &self,
        name: String,
        description: String,
        user_id: Uuid,
    ) -> Result<Group, DbError> {
        let mut conn = self.db.get_conn()?;
        let result = conn.transaction::<Group, DbError, _>(|conn| {
            let new_group = NewGroup { name, description };

            let group_result = diesel::insert_into(group::table)
                .values(&new_group)
                .returning(Group::as_returning())
                .get_result(conn)?;

            let new_user_in_group = NewUserInGroup {
                user_id,
                group_id: group_result.id,
                role: Some(MyGroupRole::Admin),
            };

            let _user_in_group_result = diesel::insert_into(user_in_group::table)
                .values(&new_user_in_group)
                .returning(UserInGroup::as_returning())
                .get_result(conn);
            Ok(group_result)
        });
        Ok(result?)
    }
    fn find_by_id(&self, id: Uuid) -> Result<Option<Group>, DbError> {
        let mut conn = self.db.get_conn()?;
        let result = group::table
            .filter(group::id.eq(id))
            .first::<Group>(&mut conn)
            .optional()?;
        Ok(result)
    }
    fn is_member(&self, user_id: Uuid, group_id: Uuid) -> Result<bool, DbError> {
        let mut conn = self.db.get_conn()?;
        let result = user_in_group::table
            .filter(user_in_group::group_id.eq(group_id))
            .filter(user_in_group::user_id.eq(user_id))
            .first::<UserInGroup>(&mut conn)
            .optional()?;
        Ok(result.is_some())
    }

    fn is_admin(&self, user_id: Uuid, group_id: Uuid) -> Result<bool, DbError> {
        let mut conn = self.db.get_conn()?;
        let result = user_in_group::table
            .filter(user_in_group::group_id.eq(group_id))
            .filter(user_in_group::user_id.eq(user_id))
            .filter(user_in_group::role.eq(MyGroupRole::Admin))
            .first::<UserInGroup>(&mut conn)
            .optional()?;
        Ok(result.is_some())
    }
    fn make_admin(&self, user_id: Uuid, group_id: Uuid) -> Result<UserInGroup, DbError> {
        let mut conn = self.db.get_conn()?;

        let result = diesel::update(user_in_group::table)
            .filter(user_in_group::group_id.eq(group_id))
            .filter(user_in_group::user_id.eq(user_id))
            .set(user_in_group::role.eq(MyGroupRole::Admin))
            .get_result::<UserInGroup>(&mut conn)?;

        Ok(result)
    }
    fn add_user_to_group(&self, user_id: Uuid, group_id: Uuid) -> Result<UserInGroup, DbError> {
        let mut conn = self.db.get_conn()?;
        let new_user_in_group = NewUserInGroup {
            user_id,
            group_id,
            role: Some(MyGroupRole::Member),
        };
        let result = diesel::insert_into(user_in_group::table)
            .values(&new_user_in_group)
            .returning(UserInGroup::as_returning())
            .get_result(&mut conn);
        Ok(result?) //aca devuelvo un json vacío porque sí si se quiere cambiar que se cambie
    }

    fn delete_group(&self, group_id: Uuid) -> Result<Group, DbError> {
        let mut conn = self.db.get_conn()?;
        let result = diesel::update(group::table.filter(group::id.eq(group_id)))
            .set(group::status.eq(MyGroupStatus::Ended))
            .get_result::<Group>(&mut conn)?;
        Ok(result)
    }
    fn is_group_active(&self, group_id: Uuid) -> Result<bool, DbError> {
        let mut conn = self.db.get_conn()?;
        let result = group::table
            .filter(group::id.eq(group_id))
            .filter(group::status.eq(MyGroupStatus::Active))
            .first::<Group>(&mut conn)
            .optional()?;
        Ok(result.is_some())
    }

    fn get_group_members(&self, group_id: Uuid) -> Result<Vec<GroupMember>, DbError> {
        let mut conn = self.db.get_conn()?;
        let raw_result = user_in_group::table
            .inner_join(user::table)
            .filter(user_in_group::group_id.eq(group_id))
            .filter(user_in_group::status.eq(MyGroupMemberStatus::Active))
            .get_results::<(UserInGroup, User)>(&mut conn)?;

        let members = raw_result
            .into_iter()
            .map(|(rel, u)| GroupMember {
                user_id: u.id,
                group_id: rel.group_id,
                name: u.name,
                email: u.email,
                status: rel.status,
                role: rel.role,
            })
            .collect();

        Ok(members)
    }

    fn get_user_groups(&self, user_id: Uuid) -> Result<Vec<GroupFromUser>, DbError> {
        let mut conn = self.db.get_conn()?;

        let raw_results = group::table
            .inner_join(user_in_group::table)
            .filter(user_in_group::user_id.eq(user_id))
            .filter(user_in_group::status.eq(MyGroupMemberStatus::Active))
            .get_results::<(Group, UserInGroup)>(&mut conn)?;

        let groups = raw_results
            .into_iter()
            .map(|(g, rel)| GroupFromUser {
                user_id: rel.user_id,
                group_id: g.id,
                group_name: g.name,
                group_description: g.description,
                status: g.status,
            })
            .collect();

        Ok(groups)
    }
    fn update_group_info(
        &self,
        group_id: Uuid,
        group_update: GroupUpdate,
    ) -> Result<Group, DbError> {
        let mut conn = self.db.get_conn()?;

        let result = diesel::update(group::table)
            .filter(group::id.eq(group_id))
            .set((
                &group_update,
                group::updated_at.eq(chrono::Utc::now().naive_utc().date()),
            ))
            .get_result::<Group>(&mut conn)?;
        Ok(result)
    }
}
