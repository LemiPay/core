use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, Selectable};
use diesel_derive_enum::DbEnum;
use serde::Serialize;
use uuid::Uuid;

use crate::domain::treasury::TransactionType;
use crate::infrastructure::db::schema;

// ----------------------------
// Currency
// ----------------------------

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = schema::currency)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CurrencyModel {
    pub currency_id: Uuid,
    pub name: String,
    pub ticker: String,
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
