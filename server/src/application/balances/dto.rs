use crate::domain::balances::BalancesMap;
use crate::domain::user::UserId;
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
impl GroupBalancesDetails {
    pub fn to_domain(&self) -> BalancesMap {
        let balances = self
            .balances
            .iter()
            .map(|b| (UserId(b.user_id), b.balance.clone()))
            .collect();

        BalancesMap::new(balances, self.group_balance.clone())
    }
}
