use std::fmt::{Display, Formatter};
use uuid::Uuid;

use crate::domain::user::UserError;

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
    pub fn new(value: String) -> Result<Self, UserError> {
        if !value.contains("@") {
            return Err(UserError::InvalidEmail);
        }

        Ok(Self(value))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserName(String);

impl Display for UserName {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl UserName {
    pub fn new(value: String) -> Result<Self, UserError> {
        if value.trim().is_empty() {
            return Err(UserError::InvalidName);
        }

        Ok(Self(value))
    }
}
