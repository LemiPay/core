use diesel::prelude::*;

use crate::application::common::repo_error::RepoError;
use crate::application::permission::traits::repository::PermissionRepository;
use crate::domain::group::GroupId;
use crate::domain::group::member::GroupRole;
use crate::domain::permission::action::Action;
use crate::infrastructure::db::{
    models::group::GroupRoleModel,
    models::permission::NewGroupPermissionModel,
    pool::{DbConn, DbPool},
    schema,
};

pub struct DieselPermissionRepository {
    db: DbPool,
}

impl DieselPermissionRepository {
    pub fn new(db: DbPool) -> Self {
        Self { db }
    }

    fn get_conn(&self) -> Result<DbConn, RepoError> {
        self.db.get().map_err(|_| RepoError::Connection)
    }
}

impl PermissionRepository for DieselPermissionRepository {
    fn is_action_allowed(
        &self,
        group_id: GroupId,
        role: GroupRole,
        action: &Action,
    ) -> Result<bool, RepoError> {
        let mut conn = self.get_conn()?;
        let role_model: GroupRoleModel = role.into();

        use diesel::select;

        let exists = select(diesel::dsl::exists(
            schema::group_permission::table
                .filter(schema::group_permission::group_id.eq(group_id.0))
                .filter(schema::group_permission::role.eq(role_model))
                .filter(schema::group_permission::action.eq(action.name())),
        ))
        .get_result::<bool>(&mut conn)
        .map_err(|_| RepoError::Query)?;

        Ok(exists)
    }

    fn find_by_group(&self, group_id: GroupId) -> Result<Vec<(GroupRole, Action)>, RepoError> {
        let mut conn = self.get_conn()?;

        let rows = schema::group_permission::table
            .filter(schema::group_permission::group_id.eq(group_id.0))
            .select((
                schema::group_permission::role,
                schema::group_permission::action,
            ))
            .load::<(GroupRoleModel, String)>(&mut conn)
            .map_err(|_| RepoError::Query)?;

        rows.into_iter()
            .map(|(role, action_str)| {
                let action = Action::from_name(&action_str).ok_or(RepoError::Query)?;
                let role: GroupRole = role.into();
                Ok((role, action))
            })
            .collect()
    }

    fn add_permission(
        &self,
        group_id: GroupId,
        role: GroupRole,
        action: &Action,
    ) -> Result<(), RepoError> {
        let mut conn = self.get_conn()?;
        let role_model: GroupRoleModel = role.into();

        diesel::insert_into(schema::group_permission::table)
            .values(&NewGroupPermissionModel {
                group_id: group_id.0,
                role: role_model,
                action: action.name().to_string(),
            })
            .on_conflict_do_nothing()
            .execute(&mut conn)
            .map_err(|_| RepoError::Insert)?;

        Ok(())
    }

    fn remove_permission(
        &self,
        group_id: GroupId,
        role: GroupRole,
        action: &Action,
    ) -> Result<(), RepoError> {
        let mut conn = self.get_conn()?;
        let role_model: GroupRoleModel = role.into();

        diesel::delete(schema::group_permission::table)
            .filter(schema::group_permission::group_id.eq(group_id.0))
            .filter(schema::group_permission::role.eq(role_model))
            .filter(schema::group_permission::action.eq(action.name()))
            .execute(&mut conn)
            .map_err(|_| RepoError::Query)?;

        Ok(())
    }

    fn seed_defaults(&self, group_id: GroupId) -> Result<(), RepoError> {
        let mut conn = self.get_conn()?;

        let rows: Vec<NewGroupPermissionModel> = Action::ALL
            .iter()
            .map(|action| NewGroupPermissionModel {
                group_id: group_id.0,
                role: GroupRoleModel::Admin,
                action: action.name().to_string(),
            })
            .chain(
                [Action::UpdateGroup]
                    .iter()
                    .map(|action| NewGroupPermissionModel {
                        group_id: group_id.0,
                        role: GroupRoleModel::Member,
                        action: action.name().to_string(),
                    }),
            )
            .collect();

        diesel::insert_into(schema::group_permission::table)
            .values(&rows)
            .on_conflict_do_nothing()
            .execute(&mut conn)
            .map_err(|_| RepoError::Insert)?;

        Ok(())
    }
}
