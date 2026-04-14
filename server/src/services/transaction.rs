use std::sync::Arc;
use uuid::Uuid;

use bigdecimal::{BigDecimal, Zero};

use crate::errors::app_error::AppError;
use crate::handlers::transaction::{
    ExecuteWithdrawRequest, FundGroupRequest, WithdrawProposalRequest,
};
use crate::models::proposal::MyProposalStatus;
use crate::models::proposals::withdraw::WithdrawProposalExpanded;
use crate::models::transaction::{MyTransactionType, NewTransaction, Transaction};
use crate::repositories::traits::proposal_repo::ProposalRepository;
use crate::repositories::traits::transaction_repo::TransactionRepository;

#[derive(Clone)]
pub struct TransactionService {
    transaction_repo: Arc<dyn TransactionRepository>,
    proposal_repo: Arc<dyn ProposalRepository>,
}

impl TransactionService {
    pub fn new(
        transaction_repo: Arc<dyn TransactionRepository>,
        proposal_repo: Arc<dyn ProposalRepository>,
    ) -> Self {
        Self {
            transaction_repo,
            proposal_repo,
        }
    }

    pub fn fund_group(
        &self,
        user_id: Uuid,
        group_id: Uuid,
        payload: FundGroupRequest,
    ) -> Result<Transaction, AppError> {
        if payload.amount <= BigDecimal::zero() {
            return Err(AppError::BadRequest("Amount must be greater than 0".into()));
        }

        let user_wallet = self
            .transaction_repo
            .get_user_wallet(user_id, payload.address.clone(), payload.currency_id)?
            .ok_or(AppError::BadRequest(
                "User does not have a wallet for this currency".into(),
            ))?;

        if user_wallet.balance < payload.amount {
            return Err(AppError::BadRequest("Insufficient balance".into()));
        }

        self.transaction_repo
            .get_group_wallet(group_id, payload.currency_id)?
            .ok_or(AppError::BadRequest(
                "Group does not have a wallet for this currency".into(),
            ))?;

        let new_tx = NewTransaction {
            tx_hash: None,
            amount: payload.amount,
            user_id,
            group_id,
            currency_id: payload.currency_id,
            address: payload.address,
            description: payload.description,
            tx_type: MyTransactionType::Deposit,
        };

        let result = self.transaction_repo.create_deposit(new_tx)?;
        Ok(result)
    }

    pub fn create_withdraw_proposal(
        &self,
        user_id: Uuid,
        group_id: Uuid,
        // unpacking ↓
        WithdrawProposalRequest {
            address,
            amount,
            currency_id,
        }: WithdrawProposalRequest,
    ) -> Result<WithdrawProposalExpanded, AppError> {
        if amount <= BigDecimal::zero() {
            return Err(AppError::BadRequest("Amount must be greater than 0".into()));
        }

        let group_wallet = self
            .transaction_repo
            .get_group_wallet(group_id, currency_id)?
            .ok_or(AppError::BadRequest(
                "Group does not have a wallet for this currency".into(),
            ))?;

        if group_wallet.balance < amount {
            return Err(AppError::BadRequest("Insufficient group balance".into()));
        }

        self.transaction_repo
            .get_user_wallet(user_id, currency_id)?
            .ok_or(AppError::BadRequest(
                "User does not have a wallet for this currency".into(),
            ))?;

        let result =
            self.proposal_repo
                .create_withdraw_proposal(user_id, group_id, currency_id, amount)?;
        Ok(result)
    }

    pub fn execute_withdraw(
        &self,
        user_id: Uuid,
        group_id: Uuid,
        // unpacking ↓
        ExecuteWithdrawRequest {
            proposal_id,
            currency_id,
        }: ExecuteWithdrawRequest,
    ) -> Result<Transaction, AppError> {
        let expanded = self
            .proposal_repo
            .find_withdraw_proposal(proposal_id, currency_id)?
            .ok_or(AppError::NotFound)?;

        let expanded_valid = match expanded.proposal.status {
            MyProposalStatus::Approved => Ok(expanded),
            MyProposalStatus::Executed => Err(AppError::BadRequest(
                "Proposal has already been executed".into(),
            )),
            _ => Err(AppError::NotFound),
        }?;

        if expanded_valid.proposal.group_id != group_id {
            return Err(AppError::Forbidden);
        }

        let group_wallet = self
            .transaction_repo
            .get_group_wallet(group_id, currency_id)?
            .ok_or(AppError::BadRequest(
                "Group does not have a wallet for this currency".into(),
            ))?;

        if group_wallet.balance < expanded_valid.withdraw_proposal.amount {
            return Err(AppError::BadRequest("Insufficient group balance".into()));
        }

        self.transaction_repo
            .get_user_wallet(user_id, currency_id)?
            .ok_or(AppError::BadRequest(
                "User does not have a wallet for this currency".into(),
            ))?;

        let new_tx = NewTransaction {
            tx_hash: None,
            amount: expanded_valid.withdraw_proposal.amount,
            user_id,
            group_id,
            currency_id,
            description: None,
            tx_type: MyTransactionType::Withdraw,
        };

        let result = self
            .transaction_repo
            .execute_withdraw(proposal_id, new_tx)?;
        Ok(result)
    }

    pub fn list_by_group(&self, group_id: Uuid) -> Result<Vec<Transaction>, AppError> {
        let result = self.transaction_repo.find_by_group(group_id)?;
        Ok(result)
    }

    pub fn get_by_id(&self, transaction_id: Uuid, group_id: Uuid) -> Result<Transaction, AppError> {
        let tx = self
            .transaction_repo
            .find_by_id(transaction_id)?
            .ok_or(AppError::NotFound)?;

        if tx.group_id != group_id {
            return Err(AppError::Forbidden);
        }

        Ok(tx)
    }
}
