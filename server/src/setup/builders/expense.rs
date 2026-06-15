use std::sync::Arc;

use crate::{
    application::expense::ExpenseService,
    infrastructure::db::repositories::{
        expense_repo_impl::DieselExpenseRepository, group_repo_impl::DieselGroupRepository,
    },
};

pub fn build_expense_service(
    group_repo: Arc<DieselGroupRepository>,
    expense_repo: Arc<DieselExpenseRepository>,
) -> ExpenseService {
    ExpenseService {
        group_repo,
        expense_repo,
    }
}
