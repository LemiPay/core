use std::sync::Arc;

use bigdecimal::BigDecimal;

use crate::application::treasury::dto::UserWalletDetails;
use crate::application::treasury::traits::user_wallet_repo::UserWalletRepository;
use crate::domain::treasury::{Money, TreasuryError, UserWalletId};
use crate::domain::user::UserId;

#[derive(Debug)]
pub enum FaucetWithdrawWalletError {
    InvalidAmount,
    NotFound,
    NotOwner,
    InsufficientFunds,
    Internal,
}

#[derive(Clone)]
pub struct FaucetWithdrawWalletUseCase {
    pub user_wallet_repo: Arc<dyn UserWalletRepository>,
}

impl FaucetWithdrawWalletUseCase {
    pub fn execute(
        &self,
        user_id: UserId,
        wallet_id: UserWalletId,
        amount: BigDecimal,
    ) -> Result<UserWalletDetails, FaucetWithdrawWalletError> {
        let wallet = self
            .user_wallet_repo
            .find_by_id(wallet_id)
            .map_err(|_| FaucetWithdrawWalletError::Internal)?
            .ok_or(FaucetWithdrawWalletError::NotFound)?;

        if !wallet.is_owned_by(user_id) {
            return Err(FaucetWithdrawWalletError::NotOwner);
        }

        let money = Money::positive(amount, wallet.balance.currency).map_err(|err| match err {
            TreasuryError::InvalidAmount => FaucetWithdrawWalletError::InvalidAmount,
            _ => FaucetWithdrawWalletError::Internal,
        })?;

        let updated = wallet.withdraw(&money).map_err(|err| match err {
            TreasuryError::InsufficientFunds => FaucetWithdrawWalletError::InsufficientFunds,
            _ => FaucetWithdrawWalletError::Internal,
        })?;

        self.user_wallet_repo
            .save(&updated)
            .map_err(|_| FaucetWithdrawWalletError::Internal)?;

        self.user_wallet_repo
            .get_details(updated.id)
            .map_err(|_| FaucetWithdrawWalletError::Internal)?
            .ok_or(FaucetWithdrawWalletError::Internal)
    }
}
