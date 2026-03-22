use crate::data::error::DbError;
use crate::models::user::User;

pub trait AuthRepository: Send + Sync {
    fn register(&self, name: String, email: String, password: String) -> Result<User, DbError>;
}
