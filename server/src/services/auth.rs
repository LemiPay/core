use std::sync::Arc;

use crate::models::user::User;
use crate::repositories::traits::auth_repo::AuthRepository;

use crate::errors::app_error::AppError;
use crate::handlers::auth::{LoginRequest, RegisterRequest};

use crate::helpers::validations::require_non_empty;
use crate::security::password::{hash_password, verify_password};

use crate::security::jwt::generate_jwt;
use validator::{ValidateEmail, ValidateLength};

#[derive(Clone)]
pub struct AuthService {
    repo: Arc<dyn AuthRepository>,
}

impl AuthService {
    pub fn new(repo: Arc<dyn AuthRepository>) -> Self {
        Self { repo }
    }

    pub fn register_user(&self, user: RegisterRequest) -> Result<User, AppError> {
        // Validate data
        let name = require_non_empty(user.name, "Name")?;
        let email = require_non_empty(user.email, "Email")?;
        let password = require_non_empty(user.password, "Password")?;

        let valid = ValidateEmail::validate_email(&email)
            && ValidateLength::validate_length(&password, Some(5), Some(30), None)
            && ValidateLength::validate_length(&name.trim(), Some(2), Some(50), None);

        if !valid {
            return Err(AppError::BadRequest("Invalid registration data".into()));
        }

        let password_hash =
            hash_password(&password).map_err(|e| AppError::PasswordHash(e.to_string()))?;

        let user = self.repo.register(name, email, password_hash)?;

        Ok(user)
    }

    pub fn login_user(&self, user: LoginRequest) -> Result<String, AppError> {
        // Validate data
        let email = require_non_empty(user.email, "Email")?;
        let password = require_non_empty(user.password, "Password")?;

        let valid = ValidateEmail::validate_email(&email)
            && ValidateLength::validate_length(&password, Some(5), Some(30), None);

        if !valid {
            return Err(AppError::Unauthorized);
        }

        let found_user = self
            .repo
            .find_by_email(email)?
            .ok_or(AppError::Unauthorized)?;

        verify_password(&password, &found_user.password).map_err(|_| AppError::Unauthorized)?;

        let token = generate_jwt(found_user.id).map_err(|_| AppError::Internal)?;

        Ok(token)
    }
}
