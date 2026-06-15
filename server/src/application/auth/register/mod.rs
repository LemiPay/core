use std::sync::Arc;

use crate::application::auth::new_user::NewUser;
use crate::application::auth::{
    error::AuthError,
    register::dto::{RegisterInput, RegisterOutput},
    traits::{password_hasher::PasswordHasher, repository::AuthRepository},
};
use crate::application::notifications::repository::NotificationRepository;
use crate::application::users::traits::repository::UserRepository;
use crate::domain::user::{Email, UserName};

pub mod dto;

#[derive(Clone)]
pub struct RegisterUseCase {
    pub auth_repo: Arc<dyn AuthRepository>,
    pub user_repo: Arc<dyn UserRepository>,
    pub hash_service: Arc<dyn PasswordHasher>,
    pub notification_repo: Arc<dyn NotificationRepository>,
}

impl RegisterUseCase {
    pub fn execute(&self, input: RegisterInput) -> Result<RegisterOutput, AuthError> {
        let email = Email::new(input.email).map_err(|_| AuthError::InvalidEmail)?;
        let name = UserName::new(input.name).map_err(|_| AuthError::InvalidName)?;

        if self
            .user_repo
            .find_by_email(&email)
            .map_err(|_| AuthError::InternalError)?
            .is_some()
        {
            return Err(AuthError::EmailAlreadyExists);
        }

        if input.password.is_empty() {
            return Err(AuthError::InvalidCredentials);
        }

        let password_hash = self
            .hash_service
            .hash(&input.password)
            .map_err(|_| AuthError::InternalError)?;

        let auth_user = NewUser {
            email: email.to_string(),
            password: Some(password_hash),
            name: name.to_string(),
        };

        let saved_user = self
            .auth_repo
            .save(&auth_user)
            .map_err(|_| AuthError::InternalError)?;

        // Create explicit default notification preference rows for the new user (all true)
        let _ = self
            .notification_repo
            .initialize_defaults_for_user(saved_user.user.id);

        Ok(RegisterOutput {
            user_id: saved_user.user.id,
        })
    }
}
