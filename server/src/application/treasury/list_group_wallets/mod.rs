use std::sync::Arc;

use crate::application::treasury::dto::GroupWalletDetails;
use crate::application::treasury::traits::group_wallet_repo::GroupWalletRepository;
use crate::domain::group::GroupId;

#[derive(Debug)]
pub enum ListGroupWalletsError {
    Internal,
}

#[derive(Clone)]
pub struct ListGroupWalletsUseCase {
    pub group_wallet_repo: Arc<dyn GroupWalletRepository>,
}

impl ListGroupWalletsUseCase {
    pub fn execute(
        &self,
        group_id: GroupId,
    ) -> Result<Vec<GroupWalletDetails>, ListGroupWalletsError> {
        self.group_wallet_repo
            .list_details_by_group(group_id)
            .map_err(|_| ListGroupWalletsError::Internal)
    }
}
