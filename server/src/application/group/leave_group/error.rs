use crate::domain::group::GroupError;

#[derive(Debug)]
pub enum LeaveGroupError {
    NotFound,
    NotMember,
    LastAdminCannotLeave,
    InternalError,
}

impl From<GroupError> for LeaveGroupError {
    fn from(err: GroupError) -> Self {
        match err {
            GroupError::NotMember => LeaveGroupError::NotMember,
            GroupError::LastAdminCannotLeave => LeaveGroupError::LastAdminCannotLeave,
            _ => LeaveGroupError::InternalError,
        }
    }
}
