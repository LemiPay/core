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
    fn register(&self, name: String, email: String, password: String) -> Result<User, DbError> {
        let mut conn = self.db.get_conn()?;

        let new_user = NewUser {
            name,
            email,
            password,
        };

        let result = diesel::insert_into(user::table)
            .values(&new_user)
            .returning(User::as_returning())
            .get_result(&mut conn)?;

        Ok(result)
    }
}
