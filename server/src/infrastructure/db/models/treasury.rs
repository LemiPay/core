use alloy::primitives::Address;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, Selectable};
use diesel_derive_enum::DbEnum;
use serde::Serialize;
use uuid::Uuid;

use crate::domain::treasury::{Currency, TransactionType};
use crate::infrastructure::db::schema;

// ----------------------------
// Currency
// ----------------------------

#[derive(Debug, DbEnum, Clone, Copy, PartialEq, Eq, Serialize)]
#[db_enum(existing_type_path = "crate::infrastructure::db::schema::sql_types::Blockchain")]
pub enum BlockchainModel {
    Ethereum,
    Sepolia,
    Arbitrum,
    Base,
    Polygon,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = schema::currency)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CurrencyModel {
    pub currency_id: Uuid,
    pub name: String,
    pub ticker: String,
    pub blockchain: BlockchainModel,
    pub token_address: String,
    pub decimals: i16,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
}

// ----------------------------
// User wallet
// ----------------------------

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = schema::user_wallet)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserWalletModel {
    pub id: Uuid,
    pub address: String,
    pub user_id: Uuid,
    pub currency_id: Uuid,
    pub balance: BigDecimal,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = schema::user_wallet)]
pub struct NewUserWalletModel {
    pub id: Uuid,
    pub address: String,
    pub user_id: Uuid,
    pub currency_id: Uuid,
    pub balance: BigDecimal,
}

// ----------------------------
// Group wallet
// ----------------------------

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = schema::group_wallet)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GroupWalletModel {
    pub id: Uuid,
    pub address: String,
    pub group_id: Uuid,
    pub currency_id: Uuid,
    pub balance: BigDecimal,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = schema::group_wallet)]
pub struct NewGroupWalletModel {
    pub id: Uuid,
    pub address: String,
    pub group_id: Uuid,
    pub currency_id: Uuid,
    pub balance: BigDecimal,
}

// ----------------------------
// Transaction
// ----------------------------

#[derive(Debug, DbEnum, Clone, Copy, PartialEq, Eq, Serialize)]
#[db_enum(existing_type_path = "crate::infrastructure::db::schema::sql_types::TransactionType")]
pub enum TransactionTypeModel {
    Deposit,
    Withdraw,
    Expense,
    Investment,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = schema::transaction)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TransactionModel {
    pub id: Uuid,
    pub tx_hash: Option<String>,
    pub amount: BigDecimal,
    pub user_id: Uuid,
    pub group_id: Uuid,
    pub currency_id: Uuid,
    pub address: String,
    pub description: Option<String>,
    pub tx_type: TransactionTypeModel,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = schema::transaction)]
pub struct NewTransactionModel {
    pub tx_hash: Option<String>,
    pub amount: BigDecimal,
    pub user_id: Uuid,
    pub group_id: Uuid,
    pub currency_id: Uuid,
    pub address: String,
    pub description: Option<String>,
    pub tx_type: TransactionTypeModel,
}

// ----------------------------
// Mappings DB <-> domain
// ----------------------------

impl From<TransactionTypeModel> for TransactionType {
    fn from(value: TransactionTypeModel) -> Self {
        match value {
            TransactionTypeModel::Deposit => TransactionType::Deposit,
            TransactionTypeModel::Withdraw => TransactionType::Withdraw,
            TransactionTypeModel::Expense => TransactionType::Expense,
            TransactionTypeModel::Investment => TransactionType::Investment,
        }
    }
}

impl From<TransactionType> for TransactionTypeModel {
    fn from(value: TransactionType) -> Self {
        match value {
            TransactionType::Deposit => TransactionTypeModel::Deposit,
            TransactionType::Withdraw => TransactionTypeModel::Withdraw,
            TransactionType::Expense => TransactionTypeModel::Expense,
            TransactionType::Investment => TransactionTypeModel::Investment,
        }
    }
}

// ----------------------------
// Blockchain event
// ----------------------------

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = schema::blockchain_event)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BlockchainEventModel {
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
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = schema::blockchain_event)]
pub struct NewBlockchainEventModel {
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
}

// ----------------------------
// Blockchain sync state
// ----------------------------

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = schema::blockchain_sync_state)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct BlockchainSyncStateModel {
    pub sync_key: String,
    pub last_processed_block: i64,
    pub updated_at: chrono::NaiveDateTime,
}

pub struct UpdateSyncStateModel {
    pub last_processed_block: i64,
    pub updated_at: chrono::NaiveDateTime,
}

impl From<CurrencyModel> for Currency {
    fn from(value: CurrencyModel) -> Self {
        Currency {
            id: crate::domain::treasury::currency::CurrencyId(value.currency_id),
            name: value.name,
            ticker: value.ticker,
            blockchain: match value.blockchain {
                BlockchainModel::Ethereum => {
                    crate::domain::treasury::currency::Blockchain::Ethereum
                }
                BlockchainModel::Sepolia => crate::domain::treasury::currency::Blockchain::Sepolia,
                BlockchainModel::Arbitrum => {
                    crate::domain::treasury::currency::Blockchain::Arbitrum
                }
                BlockchainModel::Base => crate::domain::treasury::currency::Blockchain::Base,
                BlockchainModel::Polygon => crate::domain::treasury::currency::Blockchain::Polygon,
            },
            token_address: crate::domain::treasury::currency::CurrencyAddress(
                value
                    .token_address
                    .parse::<Address>()
                    .expect("valid token address"),
            ),
            token_currency_id: None,
            decimals: value.decimals,
            is_active: value.is_active,
        }
    }
}
