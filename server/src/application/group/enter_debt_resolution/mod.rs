pub mod dto;
use std::sync::Arc;

use crate::application::group::{
    enter_debt_resolution::dto::{EnterDebtResolutionInput, EnterDebtResolutionOutput},
    traits::repository::GroupRepository,
};
use crate::application::investment::traits::repository::InvestmentRepository;
use crate::domain::group::GroupError;
use crate::domain::investment::InvestmentStatus;

#[derive(Debug)]
pub enum EnterDebtResolutionError {
    NotFound,
    Forbidden,
    NotActive,
    ActiveInvestments,
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
    pub investment_repo: Arc<dyn InvestmentRepository>,
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

        let investments = self
            .investment_repo
            .list_group_investments(input.group_id.0)
            .map_err(|_| EnterDebtResolutionError::Internal)?;

        let has_non_withdrawn = investments
            .iter()
            .any(|inv| inv.status != InvestmentStatus::Withdrawn);

        if has_non_withdrawn {
            return Err(EnterDebtResolutionError::ActiveInvestments);
        }

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
