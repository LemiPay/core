use diesel::prelude::*;
use uuid::Uuid;

use crate::data::database::Db;
use crate::data::error::DbError;
use crate::models::user::{NewUser, User, UserSummary};
use crate::repositories::traits::user_repo::UserRepository;
use crate::schema::user;

pub struct DieselUserRepository {
    db: Db,
}

impl DieselUserRepository {
    pub fn new(db: Db) -> Self {
        Self { db }
    }
}

impl UserRepository for DieselUserRepository {
    fn create(&self, name: String, email: String) -> Result<User, DbError> {
        let mut conn = self.db.get_conn()?;

        let password = "hashed_password".to_string(); // In a real application, you would hash the password properly

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

    fn find_by_id(&self, id: Uuid) -> Result<Option<UserSummary>, DbError> {
        let mut conn = self.db.get_conn()?;

        let result = user::table
            .filter(user::id.eq(id))
            .select(User::as_select())
            .first::<User>(&mut conn)
            .optional()?;
        let user_summary = result.map(|u| UserSummary {
            id: u.id,
            email: u.email,
            name: u.name,
        });

        Ok(user_summary)
    }

    fn list(&self) -> Result<Vec<User>, DbError> {
        let mut conn = self.db.get_conn()?;

        let result = user::table.load::<User>(&mut conn)?;

        Ok(result)
    }
}
