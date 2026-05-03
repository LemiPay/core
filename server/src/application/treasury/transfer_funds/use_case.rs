use std::sync::Arc;

use crate::application::treasury::traits::user_wallet_repo::UserWalletRepository;
use crate::application::treasury::transfer_funds::dto::TransferFundsInput;
use crate::application::treasury::transfer_funds::error::TransferFundsError;
use crate::domain::treasury::{Money, TreasuryPolicy};

#[derive(Clone)]
pub struct TransferFundsUseCase {
    pub user_wallet_repo: Arc<dyn UserWalletRepository>,
}

impl TransferFundsUseCase {
    pub fn execute(&self, input: TransferFundsInput) -> Result<(), TransferFundsError> {
        let sender = self
            .user_wallet_repo
            .find_by_id(input.sender_wallet_id)
            .map_err(|_| TransferFundsError::Internal)?
            .ok_or(TransferFundsError::SenderWalletNotFound)?;

        if !sender.is_owned_by(input.user_id) {
            return Err(TransferFundsError::NotOwner);
        }

        let amount = Money::positive(input.amount, sender.balance.currency)?;

        TreasuryPolicy::ensure_user_can_cover(&sender, &amount)?;
        TreasuryPolicy::ensure_distinct_addresses(&sender, &input.receiver_address)?;

        let receiver = self
            .user_wallet_repo
            .find_by_address_and_currency(&input.receiver_address, sender.balance.currency)
            .map_err(|_| TransferFundsError::Internal)?
            .ok_or(TransferFundsError::ReceiverNotFound)?;

        self.user_wallet_repo
            .transfer(sender.id, receiver.id, &amount)
            .map_err(|_| TransferFundsError::Internal)
    }
}
