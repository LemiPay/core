use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::balances::dto::{GroupBalancesDetails, UserBalanceDetails};
use crate::application::settlements::get_settlements::dto::{
    GetSettlementsOutput, SettlementDetails,
};
use crate::interfaces::http::transaction::dto::TransactionTypeResponse;

#[derive(Serialize)]
pub struct UserBalanceResponse {
    pub user_name: String,
    pub user_id: Uuid,
    pub balance: BigDecimal,
}

#[derive(Serialize)]
pub struct BalancesResponse {
    pub group_balance: BigDecimal,
    pub balances: Vec<UserBalanceResponse>,
}

impl From<UserBalanceDetails> for UserBalanceResponse {
    fn from(value: UserBalanceDetails) -> Self {
        Self {
            user_name: value.user_name,
            user_id: value.user_id,
            balance: value.balance,
        }
    }
}

impl From<GroupBalancesDetails> for BalancesResponse {
    fn from(value: GroupBalancesDetails) -> Self {
        Self {
            group_balance: value.group_balance,
            balances: value.balances.into_iter().map(Into::into).collect(),
        }
    }
}
#[derive(Serialize)]
pub struct SettlementResponse {
    pub from: Uuid,
    pub to: Uuid,
    pub amount: BigDecimal,
    pub to_name: Option<String>,
    pub from_name: Option<String>,
}

impl From<SettlementDetails> for SettlementResponse {
    fn from(value: SettlementDetails) -> Self {
        Self {
            from: value.from.0,
            to: value.to.0,
            amount: value.amount,
            to_name: value.to_name.map(|n| n.0),
            from_name: value.from_name.map(|n| n.0),
        }
    }
}

impl From<GetSettlementsOutput> for GetSettlementsResponse {
    fn from(value: GetSettlementsOutput) -> Self {
        Self {
            settlements: value.settlements.into_iter().map(Into::into).collect(),
        }
    }
}

#[derive(Serialize)]
pub struct GetSettlementsResponse {
    pub settlements: Vec<SettlementResponse>,
}

#[derive(Deserialize)]
pub struct PaySettlementRequest {
    pub amount: BigDecimal,
    pub address: String,
    pub currency_id: Uuid,
    pub description: Option<String>,
}

#[derive(Serialize)]
pub struct PaySettlementResponse {
    pub id: Uuid,
    pub amount: BigDecimal,
    pub user_id: Uuid,
    pub group_id: Uuid,
    pub currency_id: Uuid,
    pub address: String,
    pub description: Option<String>,
    pub tx_type: TransactionTypeResponse,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize)]
pub struct ClaimRequest {
    pub amount: BigDecimal,
    pub address: String,
    pub currency_id: Uuid,
    pub description: Option<String>,
}

#[derive(Serialize)]
pub struct ClaimResponse {
    pub id: Uuid,
    pub amount: BigDecimal,
    pub user_id: Uuid,
    pub group_id: Uuid,
    pub currency_id: Uuid,
    pub address: String,
    pub description: Option<String>,
    pub tx_type: TransactionTypeResponse,
    pub created_at: NaiveDateTime,
}
