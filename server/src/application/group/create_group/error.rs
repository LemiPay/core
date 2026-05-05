use crate::domain::group::GroupError;

#[derive(Debug)]
pub enum CreateGroupError {
    InvalidName,
    InvalidDescription,
    InternalError,
}

impl From<GroupError> for CreateGroupError {
    fn from(err: GroupError) -> Self {
        match err {
            GroupError::InvalidName => CreateGroupError::InvalidName,
            GroupError::InvalidDescription => CreateGroupError::InvalidDescription,
            _ => CreateGroupError::InternalError,
        }
    }
}
