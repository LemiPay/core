use std::sync::Arc;

use crate::application::balances::BalancesService;
use crate::{
    application::group::{dto::GroupDetails, traits::repository::GroupRepository},
    domain::{group::GroupId, user::UserId},
};

#[derive(Debug)]
pub enum DeleteGroupError {
    Forbidden,
    NotFound,
    Internal,
    NotAllBalancesZero,
}

#[derive(Clone)]
pub struct DeleteGroupUseCase {
    pub group_repo: Arc<dyn GroupRepository>,
    pub balances_service: BalancesService,
}

impl DeleteGroupUseCase {
    pub fn execute(
        &self,
        actor_id: UserId,
        group_id: GroupId,
    ) -> Result<GroupDetails, DeleteGroupError> {
        if !self
            .group_repo
            .is_admin(actor_id, group_id)
            .map_err(|_| DeleteGroupError::Internal)?
        {
            return Err(DeleteGroupError::Forbidden);
        }

        let group = self
            .group_repo
            .find_by_id(group_id)
            .map_err(|_| DeleteGroupError::Internal)?
            .ok_or(DeleteGroupError::NotFound)?;

        let balances = self
            .balances_service
            .get_balances(group_id)
            .map_err(|_| DeleteGroupError::Internal)?;

        let deactivated = group
            .deactivate(balances.to_domain())
            .map_err(|_| DeleteGroupError::NotAllBalancesZero)?;

        self.group_repo
            .save(&deactivated)
            .map_err(|_| DeleteGroupError::Internal)?;

        self.group_repo
            .get_group_details(group_id)
            .map_err(|_| DeleteGroupError::Internal)?
            .ok_or(DeleteGroupError::NotFound)
    }
}
