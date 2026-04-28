use std::sync::Arc;

use crate::application::users::{
    error::UserError, me::dto::MeOutput, traits::repository::UserRepository,
};

use crate::domain::user::{Email, UserId};

mod dto;

#[derive(Clone)]
pub struct GetMeUseCase {
    pub repo: Arc<dyn UserRepository>,
}

impl GetMeUseCase {
    pub fn execute(&self, user_id: UserId) -> Result<MeOutput, UserError> {
        let found = self
            .repo
            .find_by_id(&user_id)
            .map_err(|_| UserError::InternalError)?
            .ok_or(UserError::NotFound)?;

        Ok(MeOutput {
            user_id,
            email: Email(found.email),
            name: found.name,
        })
    }
}
