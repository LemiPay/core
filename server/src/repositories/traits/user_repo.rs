use crate::data::error::DbError;
use crate::models::user::User;
use uuid::Uuid;

pub trait UserRepository: Send + Sync {
    fn create(&self, name: String, email: String) -> Result<User, DbError>;

    fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DbError>;

    fn list(&self) -> Result<Vec<User>, DbError>;
}
