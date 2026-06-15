use std::collections::HashMap;

use crate::application::balances::BalancesService;
use crate::application::balances::dto::UserBalanceDetails;
use crate::application::settlements::get_settlements::dto::{
    GetSettlementsInput, GetSettlementsOutput, SettlementDetails,
};
use crate::application::settlements::get_settlements::error::GetSettlementError;
use crate::domain::settlements::algorithm::recommend_settlements;
use crate::domain::user::{UserId, UserName};

pub mod dto;
pub mod error;

pub struct GetSettlementsUseCase {
    pub balances_service: BalancesService,
}

impl GetSettlementsUseCase {
    pub fn execute(
        &self,
        input: GetSettlementsInput,
    ) -> Result<GetSettlementsOutput, GetSettlementError> {
        let group_id = input.group_id;
        let balances_details = self
            .balances_service
            .get_balances(group_id)
            .map_err(|_| GetSettlementError::Internal)?;
        let name_cache = Self::create_name_cache(&balances_details.balances);
        let balances_map = balances_details.to_domain();
        let settlements = recommend_settlements(&balances_map);

        let result: Vec<SettlementDetails> = settlements
            .into_iter()
            .map(|s| SettlementDetails {
                from: s.from,
                to: s.to,
                amount: s.amount,
                to_name: self.resolve_name(&s.to, &name_cache),
                from_name: self.resolve_name(&s.from, &name_cache),
            })
            .collect();

        Ok(GetSettlementsOutput {
            settlements: result,
        })
    }

    fn resolve_name(
        &self,
        id: &UserId,
        cache: &HashMap<UserId, Option<UserName>>,
    ) -> Option<UserName> {
        cache.get(id).cloned().unwrap_or(None)
    }

    fn create_name_cache(balances: &[UserBalanceDetails]) -> HashMap<UserId, Option<UserName>> {
        balances
            .iter()
            .map(|b| (UserId(b.user_id), Some(UserName(b.user_name.clone()))))
            .collect()
    }
}
