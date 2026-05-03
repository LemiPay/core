use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::treasury::TransactionType;

#[derive(Deserialize)]
pub struct FundGroupRequest {
    pub amount: BigDecimal,
    pub address: String,
    pub currency_id: Uuid,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct TransactionIdQuery {
    pub transaction_id: Uuid,
}

#[derive(Serialize)]
pub struct TransactionResponse {
    pub id: Uuid,
    pub tx_hash: Option<String>,
    pub amount: BigDecimal,
    pub user_id: Uuid,
    pub group_id: Uuid,
    pub currency_id: Uuid,
    pub address: String,
    pub description: Option<String>,
    pub tx_type: TransactionTypeResponse,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionTypeResponse {
    Deposit,
    Withdraw,
    Expense,
    Investment,
}

impl From<TransactionType> for TransactionTypeResponse {
    fn from(value: TransactionType) -> Self {
        match value {
            TransactionType::Deposit => TransactionTypeResponse::Deposit,
            TransactionType::Withdraw => TransactionTypeResponse::Withdraw,
            TransactionType::Expense => TransactionTypeResponse::Expense,
            TransactionType::Investment => TransactionTypeResponse::Investment,
        }
    }
}
