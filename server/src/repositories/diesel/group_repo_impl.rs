use crate::data::database::Db;
use crate::data::error::DbError;
use crate::models::group::{Group, NewGroup};
use crate::models::user_in_group::{MyGroupRole, NewUserInGroup, UserInGroup};
use crate::repositories::traits::group_repo::GroupRepository;
use crate::schema::group;
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
}
