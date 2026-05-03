use std::sync::Arc;

use crate::application::{
    auth::{
        error::AuthError,
        login::dto::{LoginInput, LoginOutput},
        traits::{password_hasher::PasswordHasher, token_service::TokenService},
    },
    users::traits::repository::UserRepository,
};

use crate::domain::user::{Email, UserId};

pub mod dto;

#[derive(Clone)]
pub struct LoginUseCase {
    pub user_repo: Arc<dyn UserRepository>,

    pub hash_service: Arc<dyn PasswordHasher>,
    pub token_service: Arc<dyn TokenService>,
}

impl LoginUseCase {
    pub fn execute(&self, input: LoginInput) -> Result<LoginOutput, AuthError> {
        let email: Email = Email::new(input.email).map_err(|_| AuthError::InvalidCredentials)?;

        let user = self
            .user_repo
            .find_by_email(&email)
            .map_err(|_| AuthError::InternalError)?
            .ok_or(AuthError::InvalidCredentials)?;

        let password_valid = self
            .hash_service
            .verify(&input.password, &user.password)
            .map_err(|_| AuthError::InternalError)?;

        if !password_valid {
            return Err(AuthError::InvalidCredentials);
        }

        let user_id = UserId(user.id);

        // Generate JWT
        let token = self
            .token_service
            .generate(user_id)
            .map_err(|_| AuthError::InternalError)?;

        Ok(LoginOutput { user_id, token })
    }
}
