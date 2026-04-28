use super::types::*;
use crate::domain::user::UserValidationError;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub name: UserName,
    pub email: Email,
}

impl User {
    pub fn new(id: Uuid, name: String, email: String) -> Result<Self, UserValidationError> {
        Ok(Self {
            id: UserId(id),
            name: UserName::new(name)?,
            email: Email::new(email)?,
        })
    }
}
