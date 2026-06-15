use std::sync::Arc;

use bigdecimal::BigDecimal;

use crate::application::treasury::dto::UserWalletDetails;
use crate::application::treasury::traits::user_wallet_repo::UserWalletRepository;
use crate::domain::treasury::{Money, TreasuryError, UserWalletId};
use crate::domain::user::UserId;

#[derive(Debug)]
pub enum FundWalletError {
    InvalidAmount,
    NotFound,
    NotOwner,
    Internal,
}

#[derive(Clone)]
pub struct FundWalletUseCase {
    pub user_wallet_repo: Arc<dyn UserWalletRepository>,
}

impl FundWalletUseCase {
    pub fn execute(
        &self,
        user_id: UserId,
        wallet_id: UserWalletId,
        amount: BigDecimal,
    ) -> Result<UserWalletDetails, FundWalletError> {
        let wallet = self
            .user_wallet_repo
            .find_by_id(wallet_id)
            .map_err(|_| FundWalletError::Internal)?
            .ok_or(FundWalletError::NotFound)?;

        if !wallet.is_owned_by(user_id) {
            return Err(FundWalletError::NotOwner);
        }

        let money = Money::positive(amount, wallet.balance.currency).map_err(|err| match err {
            TreasuryError::InvalidAmount => FundWalletError::InvalidAmount,
            _ => FundWalletError::Internal,
        })?;

        let updated = wallet
            .deposit(&money)
            .map_err(|_| FundWalletError::Internal)?;

        self.user_wallet_repo
            .save(&updated)
            .map_err(|_| FundWalletError::Internal)?;

        self.user_wallet_repo
            .get_details(updated.id)
            .map_err(|_| FundWalletError::Internal)?
            .ok_or(FundWalletError::Internal)
    }
}
