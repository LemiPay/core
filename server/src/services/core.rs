use crate::core::core::core;
use std::sync::Arc;
use uuid::Uuid;

use bigdecimal::{BigDecimal, Zero};

use crate::errors::app_error::AppError;
use crate::handlers::core::{Balances, UserBalance};
use crate::models::expense::Expense;
use crate::repositories::traits::group_repo::GroupRepository;
use crate::repositories::traits::transaction_repo::TransactionRepository;

#[derive(Clone)]
pub struct CoreService {
    transaction_repo: Arc<dyn TransactionRepository>,
    group_repo: Arc<dyn GroupRepository>,
}
impl CoreService {
    pub fn new(
        transaction_repo: Arc<dyn TransactionRepository>,
        group_repo: Arc<dyn GroupRepository>,
    ) -> Self {
        Self {
            transaction_repo,
            group_repo,
        }
    }

    pub fn get_balances(&self, group_id: Uuid) -> Result<Balances, AppError> {
        //get all users historically
        let historic_members = self.group_repo.get_historic_group_members(group_id)?;
        let users_ids = historic_members.iter().map(|m| m.user_id).collect();

        //get all transactions
        let transactions = self.transaction_repo.find_by_group(group_id)?;

        //get all expenses
        let expenses = vec![] as Vec<Expense>;

        //call core
        let result = core(users_ids, transactions, expenses).map_err(|_| AppError::Core)?;

        let balances = historic_members
            .iter()
            .map(|member| UserBalance {
                user_id: member.user_id,
                user_name: member.name.clone(),
                balance: result
                    .get_user_balance(&member.user_id)
                    .cloned()
                    .unwrap_or(BigDecimal::zero()),
            })
            .collect();

        Ok(Balances {
            group_balance: result.get_group_balance().clone(),
            balances,
        })
    }
}
