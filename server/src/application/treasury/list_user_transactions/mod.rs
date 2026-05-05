use std::sync::Arc;

use crate::application::treasury::dto::TransactionDetails;
use crate::application::treasury::traits::transaction_repo::TransactionRepository;
use crate::domain::user::UserId;

#[derive(Debug)]
pub enum ListUserTransactionsError {
    Internal,
}

#[derive(Clone)]
pub struct ListUserTransactionsUseCase {
    pub transaction_repo: Arc<dyn TransactionRepository>,
}
impl ListUserTransactionsUseCase {
    pub fn execute(
        &self,
        user_id: UserId,
    ) -> Result<Vec<TransactionDetails>, ListUserTransactionsError> {
        self.transaction_repo
            .list_by_user(user_id)
            .map_err(|_| ListUserTransactionsError::Internal)
    }
}
