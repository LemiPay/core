use crate::application::auth::new_user::NewUser;
use crate::application::{auth::stored_user::StoredUser, common::repo_error::RepoError};

pub trait AuthRepository: Send + Sync {
    fn save(&self, user: &NewUser) -> Result<StoredUser, RepoError>;
}
