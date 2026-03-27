use crate::data::error::DbError;
use crate::models::user::User;

pub trait AuthRepository: Send + Sync {
    fn register(
        &self,
        name: String,
        email: String,
        hashed_password: String,
    ) -> Result<User, DbError>;

    fn find_by_email(&self, user_email: String) -> Result<Option<User>, DbError>;
}
