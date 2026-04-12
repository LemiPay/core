use std::sync::Arc;
use uuid::Uuid;

use bigdecimal::BigDecimal;

use crate::errors::app_error::AppError;
use crate::handlers::transaction::FundGroupRequest;
use crate::models::transaction::{MyTransactionType, NewTransaction, Transaction};
use crate::repositories::traits::transaction_repo::TransactionRepository;

#[derive(Clone)]
pub struct TransactionService {
    transaction_repo: Arc<dyn TransactionRepository>,
}

impl TransactionService {
    pub fn new(transaction_repo: Arc<dyn TransactionRepository>) -> Self {
        Self { transaction_repo }
    }

    pub fn fund_group(
        &self,
        user_id: Uuid,
        group_id: Uuid,
        payload: FundGroupRequest,
    ) -> Result<Transaction, AppError> {
        if payload.amount <= BigDecimal::from(0) {
            return Err(AppError::BadRequest("Amount must be greater than 0".into()));
        }

        let user_wallet = self
            .transaction_repo
            .get_user_wallet(user_id, payload.currency_id)?
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
            tx_hash: None, // TODO: add tx_hash when implemented
            amount: payload.amount,
            user_id,
            group_id,
            currency_id: payload.currency_id,
            description: payload.description,
            tx_type: MyTransactionType::Deposit,
        };

        let result = self.transaction_repo.create_deposit(new_tx)?;
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
