use std::sync::Arc;

use crate::application::group::traits::repository::GroupRepository;
use crate::application::settlements::pay_settlement::dto::{
    PaySettlementInput, PaySettlementOutput,
};
use crate::application::settlements::pay_settlement::error::PaySettlementError;
use crate::application::treasury::dto::TransactionDetails;
use crate::application::treasury::traits::{
    group_wallet_repo::GroupWalletRepository, transaction_repo::TransactionRepository,
    user_wallet_repo::UserWalletRepository,
};
use crate::domain::group::GroupPolicy;
use crate::domain::treasury::{Money, NewTransaction, TransactionType, TreasuryPolicy};

#[derive(Clone)]
pub struct PaySettlementUseCase {
    pub group_repo: Arc<dyn GroupRepository>,
    pub user_wallet_repo: Arc<dyn UserWalletRepository>,
    pub group_wallet_repo: Arc<dyn GroupWalletRepository>,
    pub transaction_repo: Arc<dyn TransactionRepository>,
}

impl PaySettlementUseCase {
    pub fn execute(
        &self,
        input: PaySettlementInput,
    ) -> Result<PaySettlementOutput, PaySettlementError> {
        let group = self
            .group_repo
            .find_by_id(input.group_id)
            .map_err(|_| PaySettlementError::Internal)?
            .ok_or(PaySettlementError::GroupNotFound)?;
        GroupPolicy::ensure_active(&group).map_err(|_| PaySettlementError::GroupNotActive)?;
        let amount = Money::positive(input.amount, input.currency_id)?;

        let user_wallet = self
            .user_wallet_repo
            .find_by_address_and_currency(&input.address, input.currency_id)
            .map_err(|_| PaySettlementError::Internal)?
            .ok_or(PaySettlementError::UserWalletNotFound)?;

        if !user_wallet.is_owned_by(input.user_id) {
            return Err(PaySettlementError::UserWalletNotFound);
        }

        TreasuryPolicy::ensure_user_can_cover(&user_wallet, &amount)?;

        self.group_wallet_repo
            .find_by_group_and_currency(input.group_id, input.currency_id)
            .map_err(|_| PaySettlementError::Internal)?
            .ok_or(PaySettlementError::GroupWalletNotFound)?;

        let new_tx = NewTransaction {
            tx_hash: None,
            amount,
            user_id: input.user_id,
            group_id: input.group_id,
            currency_id: input.currency_id,
            address: input.address,
            description: input.description,
            tx_type: TransactionType::SettlementPayment,
        };

        let transaction: TransactionDetails = self
            .transaction_repo
            .create_user_to_group_deposit(new_tx)
            .map_err(|_| PaySettlementError::Internal)?;

        Ok(PaySettlementOutput { transaction })
    }
}
