use std::sync::Arc;

use bigdecimal::{BigDecimal, Zero};

use crate::application::balances::BalancesService;
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
    pub balances_service: BalancesService,
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
        GroupPolicy::ensure_in_debt_resolution(&group)
            .map_err(|_| PaySettlementError::GroupNotInDebtResolution)?;

        let balances_details = self
            .balances_service
            .get_balances(input.group_id)
            .map_err(|_| PaySettlementError::Internal)?;

        let user_balance = balances_details
            .balances
            .iter()
            .find(|b| b.user_id == input.user_id.0)
            .map(|b| &b.balance)
            .ok_or(PaySettlementError::Internal)?;

        if *user_balance >= BigDecimal::zero() {
            return Err(PaySettlementError::NoDebt);
        }

        if input.amount > -user_balance.clone() {
            return Err(PaySettlementError::AmountExceedsDebt);
        }

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
