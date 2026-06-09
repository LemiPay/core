use crate::application::balances::BalancesService;
use crate::application::group::leave_group::dto::{LeaveGroupInput, LeaveGroupOutput};
use crate::application::group::leave_group::error::LeaveGroupError;
use crate::application::group::traits::repository::GroupRepository;
use std::sync::Arc;

#[derive(Clone)]
pub struct LeaveGroupUseCase {
    pub group_repo: Arc<dyn GroupRepository>,
    pub balances_service: BalancesService,
}

impl LeaveGroupUseCase {
    pub fn execute(&self, input: LeaveGroupInput) -> Result<LeaveGroupOutput, LeaveGroupError> {
        let group = self
            .group_repo
            .find_by_id(input.group_id)
            .map_err(|_| LeaveGroupError::InternalError)?
            .ok_or(LeaveGroupError::NotFound)?;
        let balances = self
            .balances_service
            .get_balances(input.group_id)
            .map_err(|_| LeaveGroupError::InternalError)?;

        let updated = group.leave_group(input.user_id, &balances.to_domain())?;

        self.group_repo
            .save(&updated)
            .map_err(|_| LeaveGroupError::InternalError)?;

        let relation = self
            .group_repo
            .get_user_in_group(input.user_id, input.group_id)
            .map_err(|_| LeaveGroupError::InternalError)?
            .ok_or(LeaveGroupError::InternalError)?;

        Ok(LeaveGroupOutput { relation })
    }
}
