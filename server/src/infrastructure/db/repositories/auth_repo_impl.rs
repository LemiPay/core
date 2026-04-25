use diesel::prelude::*;

use crate::infrastructure::db::{
    models::user::{NewUserModel, UserModel},
    pool::{DbConn, DbPool},
    schema,
};

use crate::application::{
    auth::{repository::AuthRepository, stored_user::StoredUser},
    common::repo_error::RepoError,
};

use crate::domain::user::{Email, UserId};

pub struct DieselAuthRepository {
    db: DbPool,
}

impl DieselAuthRepository {
    pub fn new(db: DbPool) -> Self {
        Self { db }
    }

    fn get_conn(&self) -> Result<DbConn, RepoError> {
        self.db.get().map_err(|_| RepoError::Connection)
    }
}

impl AuthRepository for DieselAuthRepository {
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

    fn save(&self, user: &StoredUser) -> Result<(), RepoError> {
        let mut conn = self.get_conn()?;

        let new_user = NewUserModel {
            email: user.user.email.to_string(),
            password: user.password_hash.clone(),
            name: user.user.name.to_string(),
        };

        diesel::insert_into(schema::user::table)
            .values(&new_user)
            .returning(UserModel::as_returning())
            .get_result(&mut conn)
            .map_err(|_| RepoError::Insert)?;

        Ok(())
    }
}
