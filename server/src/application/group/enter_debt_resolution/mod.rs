pub mod dto;
use std::sync::Arc;

use crate::application::group::{
    enter_debt_resolution::dto::{EnterDebtResolutionInput, EnterDebtResolutionOutput},
    traits::repository::GroupRepository,
};
use crate::domain::group::GroupError;

#[derive(Debug)]
pub enum EnterDebtResolutionError {
    NotFound,
    Forbidden,
    NotActive,
    Internal,
}

impl From<GroupError> for EnterDebtResolutionError {
    fn from(err: GroupError) -> Self {
        match err {
            GroupError::NotMember | GroupError::NotAdmin => EnterDebtResolutionError::Forbidden,
            GroupError::GroupNotActive => EnterDebtResolutionError::NotActive,
            _ => EnterDebtResolutionError::Internal,
        }
    }
}

#[derive(Clone)]
pub struct EnterDebtResolutionUseCase {
    pub group_repo: Arc<dyn GroupRepository>,
}

impl EnterDebtResolutionUseCase {
    pub fn execute(
        &self,
        input: EnterDebtResolutionInput,
    ) -> Result<EnterDebtResolutionOutput, EnterDebtResolutionError> {
        let group = self
            .group_repo
            .find_by_id(input.group_id)
            .map_err(|_| EnterDebtResolutionError::Internal)?
            .ok_or(EnterDebtResolutionError::NotFound)?;

        let updated = group
            .enter_debt_resolution(input.actor_id)
            .map_err(EnterDebtResolutionError::from)?;

        self.group_repo
            .save(&updated)
            .map_err(|_| EnterDebtResolutionError::Internal)?;

        self.group_repo
            .get_group_details(input.group_id)
            .map_err(|_| EnterDebtResolutionError::Internal)?
            .ok_or(EnterDebtResolutionError::NotFound)
            .map(|group| EnterDebtResolutionOutput { group })
    }
}
