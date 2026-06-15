use std::fmt;

use crate::application::common::repo_error::RepoError;

#[derive(Debug)]
pub enum ProcessingError {
    Repo(RepoError),
}

impl fmt::Display for ProcessingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcessingError::Repo(e) => write!(f, "repo error: {:?}", e),
        }
    }
}

impl std::error::Error for ProcessingError {}

impl From<RepoError> for ProcessingError {
    fn from(e: RepoError) -> Self {
        ProcessingError::Repo(e)
    }
}
