use crate::data::database::Db;
use crate::data::error::DbError;
use crate::models::group::{Group, NewGroup};
use crate::repositories::traits::group_repo::GroupRepository;
use crate::schema::group;
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
    fn create_group(&self, name: String, description: String) -> Result<Group, DbError> {
        let mut conn = self.db.get_conn()?;

        let new_group = NewGroup { name, description };

        let result = diesel::insert_into(group::table)
            .values(&new_group)
            .returning(Group::as_returning())
            .get_result(&mut conn)?;

        Ok(result)
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
