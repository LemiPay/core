use std::sync::Arc;

use crate::application::treasury::dto::TransactionDetails;
use crate::application::treasury::traits::transaction_repo::TransactionRepository;
use crate::domain::group::GroupId;

#[derive(Debug)]
pub enum ListGroupTransactionsError {
    Internal,
}

#[derive(Clone)]
pub struct ListGroupTransactionsUseCase {
    pub transaction_repo: Arc<dyn TransactionRepository>,
}

impl ListGroupTransactionsUseCase {
    pub fn execute(
        &self,
        group_id: GroupId,
    ) -> Result<Vec<TransactionDetails>, ListGroupTransactionsError> {
        self.transaction_repo
            .list_by_group(group_id)
            .map_err(|_| ListGroupTransactionsError::Internal)
    }
}
