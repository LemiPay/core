use std::fmt::{Display, Formatter};

use crate::domain::user::UserValidationError;
use crate::id_type;

id_type!(UserId);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(pub String);

impl Display for Email {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Email {
    pub fn new(value: String) -> Result<Self, UserValidationError> {
        let normalized = value.trim().to_lowercase();
        if !normalized.contains('@') || normalized.starts_with('@') || normalized.ends_with('@') {
            return Err(UserValidationError::InvalidEmail);
        }

        Ok(Self(normalized))
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
