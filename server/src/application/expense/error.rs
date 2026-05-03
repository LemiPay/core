use crate::{
    application::common::repo_error::RepoError, domain::expense::ExpenseError as DomainError,
};

#[derive(Debug)]
pub enum ExpenseError {
    NotFound,
    Internal,
    InvalidAmount,
    EmptyParticipants,
    DuplicatedParticipant,
    TooManyParticipants,
    NoFieldsToUpdate,
    AlreadyDeleted,
    NotOwner,
    GroupMismatch,
}

impl From<RepoError> for ExpenseError {
    fn from(_: RepoError) -> Self {
        ExpenseError::Internal
    }
}

impl From<DomainError> for ExpenseError {
    fn from(value: DomainError) -> Self {
        match value {
            DomainError::InvalidAmount => ExpenseError::InvalidAmount,
            DomainError::EmptyParticipants => ExpenseError::EmptyParticipants,
            DomainError::DuplicatedParticipant => ExpenseError::DuplicatedParticipant,
            DomainError::TooManyParticipants => ExpenseError::TooManyParticipants,
            DomainError::NoFieldsToUpdate => ExpenseError::NoFieldsToUpdate,
            DomainError::AlreadyDeleted => ExpenseError::AlreadyDeleted,
            DomainError::NotOwner => ExpenseError::NotOwner,
            DomainError::GroupMismatch => ExpenseError::GroupMismatch,
        }
    }
}
