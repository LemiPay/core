use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;

use crate::domain::expense::status::ExpenseStatus;
use crate::domain::expense::types::ExpenseId;
use crate::domain::group::GroupId;
use crate::domain::treasury::CurrencyId;
use crate::domain::user::UserId;

#[derive(Debug, Clone)]
pub struct Expense {
    pub id: ExpenseId,
    pub user_id: UserId,
    pub group_id: GroupId,
    pub currency_id: CurrencyId,
    pub description: Option<String>,
    pub amount: BigDecimal,
    pub status: ExpenseStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Expense {
    #[allow(clippy::too_many_arguments)]
    pub fn rehydrate(
        id: ExpenseId,
        user_id: UserId,
        group_id: GroupId,
        currency_id: CurrencyId,
        description: Option<String>,
        amount: BigDecimal,
        status: ExpenseStatus,
        created_at: NaiveDateTime,
        updated_at: NaiveDateTime,
    ) -> Self {
        Self {
            id,
            user_id,
            group_id,
            currency_id,
            description,
            amount,
            status,
            created_at,
            updated_at,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExpenseParticipant {
    pub expense_id: ExpenseId,
    pub user_id: UserId,
    pub amount: BigDecimal,
}

impl ExpenseParticipant {
    pub fn new(expense_id: ExpenseId, user_id: UserId, amount: BigDecimal) -> Self {
        Self {
            expense_id,
            user_id,
            amount,
        }
    }
}
