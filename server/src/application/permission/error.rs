use thiserror::Error;

#[derive(Error, Debug)]
pub enum PermissionError {
    #[error("User is not a member of this group")]
    NotMember,

    #[error("Action not allowed for your role in this group")]
    ActionNotAllowed,

    #[error("Internal error")]
    Internal,
}
