use crate::application::{auth::stored_user::StoredUser, common::repo_error::RepoError};
use crate::infrastructure::db::models::user::NewUserModel;

pub trait AuthRepository: Send + Sync {
    fn save(&self, user: &NewUserModel) -> Result<StoredUser, RepoError>;
}
