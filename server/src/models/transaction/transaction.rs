use crate::schema::{transaction, transaction_participant};
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, DbEnum, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[db_enum(existing_type_path = "crate::schema::sql_types::TransactionType")]
pub enum MyTransactionType {
    Deposit,
    Withdraw,
    Expense,
    Investment,
}

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = transaction)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[allow(dead_code)] // TODO: remove after implemented
pub struct Transaction {
    pub id: Uuid,
    pub tx_hash: Option<String>,
    pub amount: BigDecimal,

    pub user_id: Uuid,
    pub group_id: Uuid,
    pub currency_id: Uuid,
    pub address: String,

    pub description: Option<String>,

    pub tx_type: MyTransactionType,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = transaction)]
pub struct NewTransaction {
    pub tx_hash: Option<String>,
    pub amount: BigDecimal,

    pub user_id: Uuid,
    pub group_id: Uuid,
    pub currency_id: Uuid,
    pub address: String,

    pub description: Option<String>,

    pub tx_type: MyTransactionType,
}

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = transaction_participant)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[allow(dead_code)] // TODO: remove after implemented
pub struct TransactionParticipant {
    pub transaction_id: Uuid,
    pub user_id: Uuid,

    pub amount: BigDecimal,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = transaction_participant)]
pub struct NewTransactionParticipant {
    pub transaction_id: Uuid,
    pub user_id: Uuid,

    pub amount: BigDecimal,
}
