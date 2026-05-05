use std::sync::Arc;

use crate::application::treasury::dto::UserWalletWithTickerDetails;
use crate::application::treasury::traits::user_wallet_repo::UserWalletRepository;
use crate::domain::user::UserId;

#[derive(Debug)]
pub enum ListUserWalletsError {
    Internal,
}

#[derive(Clone)]
pub struct ListUserWalletsUseCase {
    pub user_wallet_repo: Arc<dyn UserWalletRepository>,
}

impl ListUserWalletsUseCase {
    pub fn execute(
        &self,
        user_id: UserId,
    ) -> Result<Vec<UserWalletWithTickerDetails>, ListUserWalletsError> {
        self.user_wallet_repo
            .list_with_ticker_by_user(user_id)
            .map_err(|_| ListUserWalletsError::Internal)
    }
}
