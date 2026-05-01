use bigdecimal::BigDecimal;
use uuid::Uuid;

use crate::application::{
    common::repo_error::RepoError,
    expense::dto::{ExpenseDetails, ExpenseUpdate, NewExpense},
};

pub trait ExpenseRepository: Send + Sync {
    fn find_by_id(&self, expense_id: Uuid) -> Result<Option<ExpenseDetails>, RepoError>;
    fn find_by_group(&self, group_id: Uuid) -> Result<Vec<ExpenseDetails>, RepoError>;
    fn create(
        &self,
        new_expense: NewExpense,
        participants: Vec<(Uuid, BigDecimal)>,
    ) -> Result<ExpenseDetails, RepoError>;
    fn update(
        &self,
        expense_id: Uuid,
        update: ExpenseUpdate,
        participants: Option<Vec<(Uuid, BigDecimal)>>,
    ) -> Result<ExpenseDetails, RepoError>;
    fn soft_delete(&self, expense_id: Uuid) -> Result<ExpenseDetails, RepoError>;
}
