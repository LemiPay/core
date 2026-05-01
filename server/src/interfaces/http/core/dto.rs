use bigdecimal::BigDecimal;
use serde::Serialize;
use uuid::Uuid;

use crate::application::balances::dto::{GroupBalancesDetails, UserBalanceDetails};

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
