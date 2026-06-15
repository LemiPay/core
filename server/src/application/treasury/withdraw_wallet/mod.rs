use std::sync::Arc;

use bigdecimal::BigDecimal;

use crate::application::common::repo_error::RepoError;
use crate::application::treasury::dto::UserWalletDetails;
use crate::application::treasury::traits::user_wallet_repo::UserWalletRepository;
use crate::domain::treasury::{Money, TreasuryError, UserWalletId};
use crate::domain::user::UserId;

#[derive(Debug)]
pub enum WithdrawWalletError {
    InvalidAmount,
    NotFound,
    NotOwner,
    InsufficientFunds,
    Internal,
}

#[derive(Clone)]
pub struct WithdrawWalletUseCase {
    pub user_wallet_repo: Arc<dyn UserWalletRepository>,
}

impl WithdrawWalletUseCase {
    pub fn execute(
        &self,
        user_id: UserId,
        wallet_id: UserWalletId,
        amount: BigDecimal,
    ) -> Result<UserWalletDetails, WithdrawWalletError> {
        let wallet = self
            .user_wallet_repo
            .find_by_id(wallet_id)
            .map_err(|_| WithdrawWalletError::Internal)?
            .ok_or(WithdrawWalletError::NotFound)?;

        if !wallet.is_owned_by(user_id) {
            return Err(WithdrawWalletError::NotOwner);
        }

        let money =
            Money::positive(amount.clone(), wallet.balance.currency).map_err(|err| match err {
                TreasuryError::InvalidAmount => WithdrawWalletError::InvalidAmount,
                _ => WithdrawWalletError::Internal,
            })?;

        self.user_wallet_repo
            .withdraw_atomic(wallet_id, &money)
            .map_err(|err| match err {
                RepoError::Insert | RepoError::Query => WithdrawWalletError::InsufficientFunds,
                _ => WithdrawWalletError::Internal,
            })
    }
}
