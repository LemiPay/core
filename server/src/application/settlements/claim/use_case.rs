use std::sync::Arc;

use bigdecimal::{BigDecimal, Zero};

use crate::application::balances::BalancesService;
use crate::application::group::traits::repository::GroupRepository;
use crate::application::settlements::claim::dto::{ClaimInput, ClaimOutput};
use crate::application::settlements::claim::error::ClaimError;
use crate::application::treasury::dto::TransactionDetails;
use crate::application::treasury::traits::{
    group_wallet_repo::GroupWalletRepository, transaction_repo::TransactionRepository,
    user_wallet_repo::UserWalletRepository,
};
use crate::domain::group::GroupPolicy;
use crate::domain::treasury::{Money, NewTransaction, TransactionType, TreasuryPolicy};

#[derive(Clone)]
pub struct ClaimUseCase {
    pub group_repo: Arc<dyn GroupRepository>,
    pub user_wallet_repo: Arc<dyn UserWalletRepository>,
    pub group_wallet_repo: Arc<dyn GroupWalletRepository>,
    pub transaction_repo: Arc<dyn TransactionRepository>,
    pub balances_service: BalancesService,
}

impl ClaimUseCase {
    pub fn execute(&self, input: ClaimInput) -> Result<ClaimOutput, ClaimError> {
        let group = self
            .group_repo
            .find_by_id(input.group_id)
            .map_err(|_| ClaimError::Internal)?
            .ok_or(ClaimError::GroupNotFound)?;
        GroupPolicy::ensure_in_debt_resolution(&group)
            .map_err(|_| ClaimError::GroupNotInDebtResolution)?;

        let balances_details = self
            .balances_service
            .get_balances(input.group_id)
            .map_err(|_| ClaimError::Internal)?;

        let user_balance = balances_details
            .balances
            .iter()
            .find(|b| b.user_id == input.user_id.0)
            .map(|b| &b.balance)
            .ok_or(ClaimError::Internal)?;

        if *user_balance <= BigDecimal::zero() {
            return Err(ClaimError::NoCredit);
        }

        if input.amount > *user_balance {
            return Err(ClaimError::AmountExceedsCredit);
        }

        let amount = Money::positive(input.amount, input.currency_id)?;

        let user_wallet = self
            .user_wallet_repo
            .find_by_address_and_currency(&input.address, input.currency_id)
            .map_err(|_| ClaimError::Internal)?
            .ok_or(ClaimError::UserWalletNotFound)?;

        if !user_wallet.is_owned_by(input.user_id) {
            return Err(ClaimError::UserWalletNotFound);
        }

        let group_wallet = self
            .group_wallet_repo
            .find_by_group_and_currency(input.group_id, input.currency_id)
            .map_err(|_| ClaimError::Internal)?
            .ok_or(ClaimError::GroupWalletNotFound)?;

        TreasuryPolicy::ensure_group_can_cover(&group_wallet, &amount)?;

        let new_tx = NewTransaction {
            tx_hash: None,
            amount,
            user_id: input.user_id,
            group_id: input.group_id,
            currency_id: input.currency_id,
            address: input.address,
            description: input.description,
            tx_type: TransactionType::Claim,
        };

        let transaction: TransactionDetails = self
            .transaction_repo
            .create_group_to_user_withdrawal(new_tx)
            .map_err(|_| ClaimError::Internal)?;

        let fresh_balances = self
            .balances_service
            .get_balances(input.group_id)
            .map_err(|_| ClaimError::Internal)?;

        let balances_map = fresh_balances.to_domain();
        if GroupPolicy::can_end_group(balances_map.clone()).is_ok() {
            let deactivated = group.deactivate(balances_map)?;
            self.group_repo
                .save(&deactivated)
                .map_err(|_| ClaimError::Internal)?;
        }

        Ok(ClaimOutput { transaction })
    }
}
