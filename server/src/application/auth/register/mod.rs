use std::sync::Arc;

use crate::application::auth::{
    error::AuthError,
    password_hasher::PasswordHasher,
    register::dto::{RegisterInput, RegisterOutput},
    repository::AuthRepository,
    stored_user::StoredUser,
};

use crate::domain::user::{Email, User, UserId, UserName};

pub mod dto;

#[derive(Clone)]
pub struct RegisterUseCase {
    pub repo: Arc<dyn AuthRepository>,
    pub hash_service: Arc<dyn PasswordHasher>,
}

impl RegisterUseCase {
    pub fn execute(&self, input: RegisterInput) -> Result<RegisterOutput, AuthError> {
        let email = Email::new(input.email).map_err(|_| AuthError::InvalidEmail)?;

        let name = UserName::new(input.name).map_err(|_| AuthError::InvalidName)?;

        if self
            .repo
            .find_by_email(&email)
            .map_err(|_| AuthError::InternalError)?
            .is_some()
        {
            return Err(AuthError::EmailAlreadyExists);
        }

        // 3. crear user
        let user = User {
            id: UserId(uuid::Uuid::new_v4()),
            name,
            email,
        };

        let password_hash = self
            .hash_service
            .hash(&input.password)
            .map_err(|_| AuthError::InternalError)?;

        let auth_user = StoredUser {
            user: user.clone(),
            password_hash,
        };

        self.repo
            .save(&auth_user)
            .map_err(|_| AuthError::InternalError)?;

        Ok(RegisterOutput { user_id: user.id })
    }
}
