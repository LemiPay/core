use crate::application::{auth::stored_user::StoredUser, common::repo_error::RepoError};

use crate::domain::user::{UserId, types::Email};

use crate::infrastructure::db::models::user::UserModel;

pub trait AuthRepository: Send + Sync {
    fn find_by_email(&self, email: &Email) -> Result<Option<UserModel>, RepoError>;

    fn find_by_id(&self, user_id: &UserId) -> Result<Option<UserModel>, RepoError>;

    fn save(&self, user: &StoredUser) -> Result<(), RepoError>;
}
