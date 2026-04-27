use std::sync::Arc;

use crate::application::auth::{
    error::AuthError, me::dto::MeOutput, traits::repository::AuthRepository,
};

use crate::domain::user::{Email, UserId};

mod dto;

#[derive(Clone)]
pub struct GetMeUseCase {
    pub repo: Arc<dyn AuthRepository>,
}

impl GetMeUseCase {
    pub fn execute(&self, user_id: UserId) -> Result<MeOutput, AuthError> {
        let found = self
            .repo
            .find_by_id(&user_id)
            .map_err(|_| AuthError::InternalError)?
            .ok_or(AuthError::InvalidCredentials)?;

        Ok(MeOutput {
            user_id,
            email: Email(found.email),
            name: found.name,
        })
    }
}
