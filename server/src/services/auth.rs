use std::sync::Arc;

use crate::models::user::User;
use crate::repositories::traits::auth_repo::AuthRepository;

use crate::errors::app_error::AppError;
use crate::handlers::auth::RegisterRequest;

#[derive(Clone)]
pub struct AuthService {
    repo: Arc<dyn AuthRepository>,
}

impl AuthService {
    pub fn new(repo: Arc<dyn AuthRepository>) -> Self {
        Self { repo }
    }

    pub fn register_user(&self, user: RegisterRequest) -> Result<User, AppError> {
        let name = user.name.ok_or(AppError::BadRequest("Name empty".into()))?;
        let email = user.email.ok_or(AppError::BadRequest("Email empty".into()))?;
        let password = user.password.ok_or(AppError::BadRequest("Password empty".into()))?;

        let user = self.repo.register(name, email, password)?;

        Ok(user)
    }

    //pub fn login_user(&self, user: Lo)
}
