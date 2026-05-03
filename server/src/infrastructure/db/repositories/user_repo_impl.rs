use diesel::prelude::*;

use crate::application::{
    common::repo_error::RepoError, users::traits::repository::UserRepository,
};

use crate::domain::user::{Email, UserId};

use crate::infrastructure::db::{
    models::user::UserModel,
    pool::{DbConn, DbPool},
    schema,
};

pub struct DieselUserRepository {
    db: DbPool,
}

impl DieselUserRepository {
    pub fn new(db: DbPool) -> Self {
        Self { db }
    }

    fn get_conn(&self) -> Result<DbConn, RepoError> {
        self.db.get().map_err(|_| RepoError::Connection)
    }
}

impl UserRepository for DieselUserRepository {
    fn find_by_email(&self, user_email: &Email) -> Result<Option<UserModel>, RepoError> {
        let mut conn = self.get_conn()?;

        let result = schema::user::table
            .filter(schema::user::email.eq(user_email.to_string()))
            .first::<UserModel>(&mut conn)
            .optional()
            .map_err(|_| RepoError::Query)?;

        Ok(result)
    }

    fn find_by_id(&self, user_id: &UserId) -> Result<Option<UserModel>, RepoError> {
        let mut conn = self.get_conn()?;

        let result = schema::user::table
            .filter(schema::user::id.eq(*user_id.as_uuid()))
            .first::<UserModel>(&mut conn)
            .optional()
            .map_err(|_| RepoError::Query)?;

        Ok(result)
    }
}
