use std::sync::Arc;
use uuid::Uuid;

use bigdecimal::BigDecimal;

use crate::errors::app_error::AppError;
use crate::handlers::core::Balances;
use crate::repositories::traits::transaction_repo::TransactionRepository;

#[derive(Clone)]
pub struct CoreService {
    transaction_repo: Arc<dyn TransactionRepository>,
}
impl CoreService {
    pub fn new(transaction_repo: Arc<dyn TransactionRepository>) -> Self {
        Self { transaction_repo }
    }

    pub fn get_balances(&self, group_id: Uuid) -> Result<Balances, AppError> {

    }
}