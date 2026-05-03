#[derive(Debug, PartialEq, Eq)]
pub enum ExpenseError {
    InvalidAmount,
    EmptyParticipants,
    DuplicatedParticipant,
    TooManyParticipants,
    NoFieldsToUpdate,
    AlreadyDeleted,
    NotOwner,
    GroupMismatch,
}
