use crate::application::common::repo_error::RepoError;

#[derive(Debug)]
pub enum FriendError {
    AlreadyExists,
    NotFound,
    SameUser,
    AlreadyFriends,
    Internal,
    InvalidAction,
    PendingRequestExists,
    CannotRequestBlocked,
}

impl From<RepoError> for FriendError {
    fn from(_: RepoError) -> Self {
        FriendError::Internal
    }
}
