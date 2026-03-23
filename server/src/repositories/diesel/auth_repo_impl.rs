use diesel::prelude::*;

use crate::data::database::Db;
use crate::data::error::DbError;
use crate::models::user::{NewUser, User};
use crate::repositories::traits::auth_repo::AuthRepository;
use crate::schema::user;

pub struct DieselAuthRepository {
    db: Db,
}

impl DieselAuthRepository {
    pub fn new(db: Db) -> Self {
        Self { db }
    }
}

impl AuthRepository for DieselAuthRepository {
    fn register(
        &self,
        name: String,
        email: String,
        hashed_password: String,
    ) -> Result<User, DbError> {
        let mut conn = self.db.get_conn()?;

        let new_user = NewUser {
            email,
            password: hashed_password,
            name,
        };

        let result = diesel::insert_into(user::table)
            .values(&new_user)
            .returning(User::as_returning())
            .get_result(&mut conn)?;

        Ok(result)
    }

    fn find_by_email(&self, user_email: String) -> Result<Option<User>, DbError> {
        let mut conn = self.db.get_conn()?;

        let result = user::table
            .filter(user::email.eq(user_email))
            .first::<User>(&mut conn)
            .optional()?;

        Ok(result)
    }
}
