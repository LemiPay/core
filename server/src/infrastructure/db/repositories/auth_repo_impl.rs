use diesel::prelude::*;

use crate::infrastructure::db::{
    models::user::{NewUserModel, UserModel},
    pool::{DbConn, DbPool},
    schema,
};

use crate::application::auth::new_user::NewUser;
use crate::application::{
    auth::{stored_user::StoredUser, traits::repository::AuthRepository},
    common::repo_error::RepoError,
};
use crate::domain::user::{Email, User, UserId, UserName};

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
    fn save(&self, user: &NewUser) -> Result<StoredUser, RepoError> {
        let mut conn = self.get_conn()?;

        let new_user = NewUserModel {
            email: user.email.clone(),
            password: user.password.clone(),
            name: user.name.clone(),
        };

        let inserted_user: UserModel = diesel::insert_into(schema::user::table)
            .values(new_user)
            .returning(UserModel::as_returning())
            .get_result(&mut conn)
            .map_err(|e| {
                println!("Error insertando usuario en DB: {:?}", e);
                RepoError::Insert
            })?;

        Ok(StoredUser {
            user: User {
                id: UserId(inserted_user.id),
                name: UserName(inserted_user.name),
                email: Email(inserted_user.email),
            },
            password_hash: inserted_user.password,
        })
    }
}
