use std::fmt::{Display, Formatter};
use uuid::Uuid;

use crate::domain::user::UserValidationError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserId(pub Uuid);

impl UserId {
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Display for UserId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(pub String);

impl Display for Email {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Email {
    pub fn new(value: String) -> Result<Self, UserValidationError> {
        if !value.contains("@") {
            return Err(UserValidationError::InvalidEmail);
        }

        Ok(Self(value))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserName(pub String);

impl Display for UserName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl UserName {
    pub fn new(value: String) -> Result<Self, UserValidationError> {
        if value.trim().is_empty() {
            return Err(UserValidationError::InvalidName);
        }

        Ok(Self(value))
    }
}
