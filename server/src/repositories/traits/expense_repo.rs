use bigdecimal::BigDecimal;
use uuid::Uuid;

use crate::data::error::DbError;
use crate::models::expense::{Expense, ExpenseUpdate, NewExpense};

pub trait ExpenseRepository: Send + Sync {
    fn find_by_id(&self, expense_id: Uuid) -> Result<Option<Expense>, DbError>;

    fn find_by_group(&self, group_id: Uuid) -> Result<Vec<Expense>, DbError>;

    fn create(
        &self,
        new_expense: NewExpense,
        participants: Vec<(Uuid, BigDecimal)>,
    ) -> Result<Expense, DbError>;

    fn update(
        &self,
        expense_id: Uuid,
        update: ExpenseUpdate,
        participants: Option<Vec<(Uuid, BigDecimal)>>,
    ) -> Result<Expense, DbError>;

    fn soft_delete(&self, expense_id: Uuid) -> Result<Expense, DbError>;
}
