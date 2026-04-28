use crate::application::{auth::stored_user::StoredUser, common::repo_error::RepoError};

pub trait AuthRepository: Send + Sync {
    fn save(&self, user: &StoredUser) -> Result<(), RepoError>;
}
