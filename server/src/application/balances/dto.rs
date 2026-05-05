use bigdecimal::BigDecimal;
use uuid::Uuid;

pub struct UserBalanceDetails {
    pub user_name: String,
    pub user_id: Uuid,
    pub balance: BigDecimal,
}

pub struct GroupBalancesDetails {
    pub group_balance: BigDecimal,
    pub balances: Vec<UserBalanceDetails>,
}
