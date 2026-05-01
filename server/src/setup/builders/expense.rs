use std::sync::Arc;

use crate::{
    application::expense::ExpenseService,
    infrastructure::db::repositories::expense_repo_impl::DieselExpenseRepository,
};

pub fn build_expense_service(expense_repo: Arc<DieselExpenseRepository>) -> ExpenseService {
    ExpenseService { expense_repo }
}
