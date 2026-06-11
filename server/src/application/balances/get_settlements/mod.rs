use crate::application::balances::BalancesService;
use crate::application::balances::get_settlements::dto::{
    GetSettlementsInput, GetSettlementsOutput, SettlementDetails,
};
use crate::application::balances::get_settlements::error::GetSettlementError;
use crate::application::users::traits::repository::UserRepository;
use crate::domain::settlements::algorithm::recommend_settlements;
use crate::domain::user::{UserId, UserName};
use std::collections::HashMap;
use std::sync::Arc;

pub mod dto;
pub mod error;

pub struct GetSettlementsUseCase {
    pub user_repo: Arc<dyn UserRepository>,
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
        let balances_map = balances_details.to_domain();
        let settlements = recommend_settlements(&balances_map);

        let mut name_cache: HashMap<UserId, Option<UserName>> = HashMap::new();
        let mut result: Vec<SettlementDetails> = Vec::new();

        for s in settlements.iter() {
            let from_name = self.resolve_name(&s.from, &mut name_cache)?;
            let to_name = self.resolve_name(&s.to, &mut name_cache)?;

            result.push(SettlementDetails {
                from: s.from.clone(),
                to: s.to.clone(),
                amount: s.amount.clone(),
                to_name,
                from_name,
            });
        }

        Ok(GetSettlementsOutput {
            settlements: result,
        })
    }
    fn resolve_name(
        &self,
        user_id: &UserId,
        name_cache: &mut HashMap<UserId, Option<UserName>>,
    ) -> Result<Option<UserName>, GetSettlementError> {
        if let Some(user_name) = name_cache.get(&user_id) {
            return Ok(user_name.clone());
        }
        let user = self
            .user_repo
            .find_by_id(user_id)
            .map_err(|_| GetSettlementError::Internal)?;
        match user {
            Some(user) => {
                let name = UserName(user.name);
                name_cache.insert(user_id.clone(), Some(name.clone()));
                Ok(Some(name))
            }
            None => {
                name_cache.insert(user_id.clone(), None);
                Ok(None)
            }
        }
    }
}
