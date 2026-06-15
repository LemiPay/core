use crate::domain::permission::action::Action;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PermissionError {
    #[error("User is not a member of this group")]
    NotMember,

    #[error("Action '{0}' is not allowed for your role in this group")]
    ActionNotAllowed(Action),

    #[error("Internal error")]
    Internal,
}
