use crate::data::database::Db;
use crate::data::error::DbError;
use crate::models::user_in_group::{NewUserInGroup, UserInGroup};
use crate::repositories::traits::user_in_group_repo::UserInGroupRepo;
use crate::schema::user_in_group;
use diesel::prelude::*;
use uuid::Uuid;

pub struct DieselUserInGroupRepository {
    db: Db,
}
impl DieselUserInGroupRepository {
    pub fn new(db: Db) -> Self {
        Self { db }
    }
}

impl UserInGroupRepo for DieselUserInGroupRepository {
    fn add_user_to_group(&self, user_id: Uuid, group_id: Uuid) -> Result<UserInGroup, DbError> {
        let mut conn = self.db.get_conn()?;
        let new_user_in_group = NewUserInGroup { user_id, group_id };
        let result = diesel::insert_into(user_in_group::table)
            .values(&new_user_in_group)
            .returning(UserInGroup::as_returning())
            .get_result(&mut conn)?;
        Ok(result)
    }
}
