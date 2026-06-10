use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::Serialize;
use uuid::Uuid;

use crate::domain::treasury::TransactionType;

pub struct UserWalletDetails {
    pub id: Uuid,
    pub address: String,
    pub user_id: Uuid,
    pub currency_id: Uuid,
    pub balance: BigDecimal,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub struct GroupWalletDetails {
    pub id: Uuid,
    pub address: String,
    pub group_id: Uuid,
    pub currency_id: Uuid,
    pub balance: BigDecimal,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub struct UserWalletWithTickerDetails {
    pub wallet_id: Uuid,
    pub address: String,
    pub balance: BigDecimal,
    pub currency_id: Uuid,
    pub ticker: String,
}

#[derive(Serialize)]
pub struct BlockchainEventDetails {
    pub id: Uuid,
    pub event_type: String,
    pub sender: String,
    pub wallet_address: String,
    pub token_address: String,
    pub currency_id: Uuid,
    pub gross_amount: BigDecimal,
    pub fee_amount: BigDecimal,
    pub net_amount: BigDecimal,
    pub tx_hash: String,
    pub block_number: i64,
    pub created_at: NaiveDateTime,
}

pub struct TransactionDetails {
    pub id: Uuid,
    pub tx_hash: Option<String>,
    pub amount: BigDecimal,
    pub user_id: Uuid,
    pub group_id: Uuid,
    pub currency_id: Uuid,
    pub address: String,
    pub description: Option<String>,
    pub tx_type: TransactionType,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
