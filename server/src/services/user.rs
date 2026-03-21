use std::sync::Arc;
use uuid::Uuid;

use crate::data::error::DbError;
use crate::models::user::User;
use crate::repositories::traits::user_repo::UserRepository;

use crate::errors::app_error::AppError;
use crate::handlers::user::CreateUserRequest;

pub struct UserService {
    repo: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self { repo }
    }

    pub fn create_user(&self, user: CreateUserRequest) -> Result<User, AppError> {
        let name = user.name.ok_or(AppError::BadRequest("Name empty".into()))?;
        let email = user.email.ok_or(AppError::BadRequest("Email empty".into()))?;

        let user = self.repo.create(name, email)?;

        Ok(user)
    }

    pub fn get_user(&self, id: Uuid) -> Result<Option<User>, DbError> {
        self.repo.find_by_id(id)
    }

    pub fn list_users(&self) -> Result<Vec<User>, DbError> {
        self.repo.list()
    }
}
