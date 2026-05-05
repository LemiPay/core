use std::sync::Arc;
pub mod dto;

use crate::application::users::{error::UserError, traits::repository::UserRepository};

use crate::domain::user::User;
use dto::{GetUserInput, GetUserOutput};

#[derive(Clone)]
pub struct UserUseCase {
    pub repo: Arc<dyn UserRepository>,
}

impl UserUseCase {
    pub fn execute(&self, input: GetUserInput) -> Result<GetUserOutput, UserError> {
        let user_model = self
            .repo
            .find_by_id(&input.user_id)
            .map_err(|_| UserError::InternalError)?;

        Ok(GetUserOutput(match user_model {
            None => None,
            Some(user) => Some(
                User::new(user.id, user.name, user.email).map_err(|_| UserError::InternalError)?,
            ),
        }))
    }
}
