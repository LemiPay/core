#[derive(Debug)]
pub enum GroupError {
    InvalidName,
    InvalidDescription,
    NotAdmin,
    NotMember,
    UserAlreadyMember,
    LastAdminCannotLeave,
    NoFieldsToUpdate,
}
