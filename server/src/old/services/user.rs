use std::sync::Arc;
use uuid::Uuid;

use crate::data::error::DbError;
use crate::models::user::{User, UserSummary};
use crate::repositories::traits::user_repo::UserRepository;

use crate::errors::app_error::AppError;
use crate::handlers::user::CreateUserRequest;

#[derive(Clone)]
pub struct UserService {
    repo: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self { repo }
    }

    pub fn create_user(&self, user: CreateUserRequest) -> Result<User, AppError> {
        let name = user
            .name
            .ok_or(AppError::BadRequest("Nombre vacío".into()))?;
        let email = user
            .email
            .ok_or(AppError::BadRequest("Email vacío".into()))?;

        let user = self.repo.create(name, email)?;

        Ok(user)
    }

    pub fn get_user(&self, id: Uuid) -> Result<Option<UserSummary>, DbError> {
        self.repo.find_by_id(id)
    }

    pub fn get_user_by_email(&self, email: String) -> Result<Option<UserSummary>, DbError> {
        self.repo.find_by_email(email)
    }

    pub fn list_users(&self) -> Result<Vec<User>, DbError> {
        self.repo.list()
    }
}
