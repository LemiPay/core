use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::domain::expense::ExpenseStatus;

pub struct ExpenseDetails {
    pub expense_id: Uuid,
    pub user_id: Uuid,
    pub currency_id: Uuid,
    pub group_id: Uuid,
    pub description: Option<String>,
    pub amount: BigDecimal,
    pub status: ExpenseStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub struct NewExpense {
    pub user_id: Uuid,
    pub group_id: Uuid,
    pub currency_id: Uuid,
    pub amount: BigDecimal,
    pub description: Option<String>,
}

pub struct ExpenseUpdate {
    pub currency_id: Option<Uuid>,
    pub amount: Option<BigDecimal>,
    pub description: Option<String>,
}

pub struct UpdateExpenseInput {
    pub currency_id: Option<Uuid>,
    pub amount: Option<String>,
    pub description: Option<String>,
    pub participants: Option<Vec<Uuid>>,
}
