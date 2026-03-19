use std::sync::Arc;
use uuid::Uuid;

use crate::data::error::DbError;
use crate::models::user::User;
use crate::repositories::traits::user_repo::UserRepository;

use crate::errors::app_error::AppError;

pub struct UserService {
    repo: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self { repo }
    }

    pub fn create_user(&self, name: String, email: String) -> Result<User, AppError> {
        if name.is_empty() || email.is_empty() {
            return Err(AppError::BadRequest("Name empty".into()));
        }

        let user = self.repo.create(name, email)?; // auto convierte DbError → AppError

        Ok(user)
    }

    pub fn get_user(&self, id: Uuid) -> Result<Option<User>, DbError> {
        self.repo.find_by_id(id)
    }

    pub fn list_users(&self) -> Result<Vec<User>, DbError> {
        self.repo.list()
    }
}
