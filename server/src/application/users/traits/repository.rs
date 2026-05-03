use crate::application::common::repo_error::RepoError;
use crate::domain::user::{Email, UserId};
use crate::infrastructure::db::models::user::UserModel;

pub trait UserRepository: Send + Sync {
    fn find_by_email(&self, email: &Email) -> Result<Option<UserModel>, RepoError>;

    fn find_by_id(&self, user_id: &UserId) -> Result<Option<UserModel>, RepoError>;
}
