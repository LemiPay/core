use std::sync::Arc;

use crate::application::treasury::dto::TransactionDetails;
use crate::application::treasury::traits::transaction_repo::TransactionRepository;
use crate::domain::group::GroupId;
use crate::domain::treasury::TransactionId;

#[derive(Debug)]
pub enum GetGroupTransactionError {
    NotFound,
    Internal,
}

#[derive(Clone)]
pub struct GetGroupTransactionUseCase {
    pub transaction_repo: Arc<dyn TransactionRepository>,
}

impl GetGroupTransactionUseCase {
    pub fn execute(
        &self,
        group_id: GroupId,
        transaction_id: TransactionId,
    ) -> Result<TransactionDetails, GetGroupTransactionError> {
        let transaction = self
            .transaction_repo
            .find_by_id(transaction_id)
            .map_err(|_| GetGroupTransactionError::Internal)?
            .ok_or(GetGroupTransactionError::NotFound)?;

        if transaction.group_id != group_id.0 {
            return Err(GetGroupTransactionError::NotFound);
        }

        Ok(transaction)
    }
}
