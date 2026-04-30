use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
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
