use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{application::expense::dto::ExpenseDetails, domain::expense::ExpenseStatus};

#[derive(Deserialize)]
pub struct ParticipantInput {
    pub user_id: Uuid,
}

#[derive(Deserialize)]
pub struct CreateExpenseRequest {
    pub currency_id: Uuid,
    pub amount: String,
    pub description: Option<String>,
    pub participants: Vec<ParticipantInput>,
}

#[derive(Deserialize)]
pub struct UpdateExpenseRequest {
    pub currency_id: Option<Uuid>,
    pub amount: Option<String>,
    pub description: Option<String>,
    pub participants: Option<Vec<ParticipantInput>>,
}

#[derive(Serialize)]
pub enum ExpenseStatusResponse {
    Created,
    Verified,
    Updated,
    Deleted,
}

impl From<ExpenseStatus> for ExpenseStatusResponse {
    fn from(value: ExpenseStatus) -> Self {
        match value {
            ExpenseStatus::Created => Self::Created,
            ExpenseStatus::Verified => Self::Verified,
            ExpenseStatus::Updated => Self::Updated,
            ExpenseStatus::Deleted => Self::Deleted,
        }
    }
}

#[derive(Serialize)]
pub struct ExpenseResponse {
    pub expense_id: Uuid,
    pub user_id: Uuid,
    pub currency_id: Uuid,
    pub group_id: Uuid,
    pub description: Option<String>,
    pub amount: BigDecimal,
    pub status: ExpenseStatusResponse,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl From<ExpenseDetails> for ExpenseResponse {
    fn from(value: ExpenseDetails) -> Self {
        Self {
            expense_id: value.expense_id,
            user_id: value.user_id,
            currency_id: value.currency_id,
            group_id: value.group_id,
            description: value.description,
            amount: value.amount,
            status: value.status.into(),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
