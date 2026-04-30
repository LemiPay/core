use std::sync::Arc;

use bigdecimal::BigDecimal;

use crate::application::treasury::dto::UserWalletDetails;
use crate::application::treasury::traits::user_wallet_repo::UserWalletRepository;
use crate::domain::treasury::{Money, TreasuryError, UserWalletId};
use crate::domain::user::UserId;

#[derive(Debug)]
pub enum FaucetFundWalletError {
    InvalidAmount,
    NotFound,
    NotOwner,
    Internal,
}

#[derive(Clone)]
pub struct FaucetFundWalletUseCase {
    pub user_wallet_repo: Arc<dyn UserWalletRepository>,
}

impl FaucetFundWalletUseCase {
    pub fn execute(
        &self,
        user_id: UserId,
        wallet_id: UserWalletId,
        amount: BigDecimal,
    ) -> Result<UserWalletDetails, FaucetFundWalletError> {
        let wallet = self
            .user_wallet_repo
            .find_by_id(wallet_id)
            .map_err(|_| FaucetFundWalletError::Internal)?
            .ok_or(FaucetFundWalletError::NotFound)?;

        if !wallet.is_owned_by(user_id) {
            return Err(FaucetFundWalletError::NotOwner);
        }

        let money = Money::positive(amount, wallet.balance.currency).map_err(|err| match err {
            TreasuryError::InvalidAmount => FaucetFundWalletError::InvalidAmount,
            _ => FaucetFundWalletError::Internal,
        })?;

        let updated = wallet
            .deposit(&money)
            .map_err(|_| FaucetFundWalletError::Internal)?;

        self.user_wallet_repo
            .save(&updated)
            .map_err(|_| FaucetFundWalletError::Internal)?;

        self.user_wallet_repo
            .get_details(updated.id)
            .map_err(|_| FaucetFundWalletError::Internal)?
            .ok_or(FaucetFundWalletError::Internal)
    }
}
