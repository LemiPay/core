use diesel::prelude::*;

use crate::infrastructure::db::{
    models::user::{NewUserModel, UserModel},
    pool::{DbConn, DbPool},
    schema,
};

use crate::application::{
    auth::{stored_user::StoredUser, traits::repository::AuthRepository},
    common::repo_error::RepoError,
};

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
