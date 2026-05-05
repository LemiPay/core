use std::sync::Arc;

use crate::application::treasury::dto::TransactionDetails;
use crate::application::treasury::fund_group::dto::{FundGroupInput, FundGroupOutput};
use crate::application::treasury::fund_group::error::FundGroupError;
use crate::application::treasury::traits::{
    group_wallet_repo::GroupWalletRepository, transaction_repo::TransactionRepository,
    user_wallet_repo::UserWalletRepository,
};
use crate::domain::treasury::{Money, NewTransaction, TransactionType, TreasuryPolicy};

#[derive(Clone)]
pub struct FundGroupUseCase {
    pub user_wallet_repo: Arc<dyn UserWalletRepository>,
    pub group_wallet_repo: Arc<dyn GroupWalletRepository>,
    pub transaction_repo: Arc<dyn TransactionRepository>,
}

impl FundGroupUseCase {
    pub fn execute(&self, input: FundGroupInput) -> Result<FundGroupOutput, FundGroupError> {
        let amount = Money::positive(input.amount, input.currency_id)?;

        let user_wallet = self
            .user_wallet_repo
            .find_by_address_and_currency(&input.address, input.currency_id)
            .map_err(|_| FundGroupError::Internal)?
            .ok_or(FundGroupError::UserWalletNotFound)?;

        if !user_wallet.is_owned_by(input.user_id) {
            return Err(FundGroupError::UserWalletNotFound);
        }

        TreasuryPolicy::ensure_user_can_cover(&user_wallet, &amount)?;

        self.group_wallet_repo
            .find_by_group_and_currency(input.group_id, input.currency_id)
            .map_err(|_| FundGroupError::Internal)?
            .ok_or(FundGroupError::GroupWalletNotFound)?;

        let new_tx = NewTransaction {
            tx_hash: None,
            amount,
            user_id: input.user_id,
            group_id: input.group_id,
            currency_id: input.currency_id,
            address: input.address,
            description: input.description,
            tx_type: TransactionType::Deposit,
        };

        let transaction: TransactionDetails = self
            .transaction_repo
            .create_user_to_group_deposit(new_tx)
            .map_err(|_| FundGroupError::Internal)?;

        Ok(FundGroupOutput { transaction })
    }
}
