use std::sync::Arc;

use crate::{
    application::balances::BalancesService,
    infrastructure::db::repositories::{
        expense_repo_impl::DieselExpenseRepository, group_repo_impl::DieselGroupRepository,
        transaction_repo_impl::DieselTransactionRepository,
    },
};

pub fn build_balances_service(
    transaction_repo: Arc<DieselTransactionRepository>,
    group_repo: Arc<DieselGroupRepository>,
    expense_repo: Arc<DieselExpenseRepository>,
) -> BalancesService {
    BalancesService {
        transaction_repo,
        group_repo,
        expense_repo,
    }
}
