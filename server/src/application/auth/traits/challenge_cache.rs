use crate::application::auth::traits::web3_auth::Web3AuthTrait;
use crate::infrastructure::auth::web_3_auth::ChallengeData;
use async_trait::async_trait;
use moka::sync::Cache;

pub trait Web3AuthCacheTrait: Web3AuthTrait + ChallengeCacheTrait {}

#[async_trait]
pub trait ChallengeCacheTrait: Send + Sync {
    fn cache_get(&self, address: &String) -> Option<ChallengeData>;

    fn cache_insert(&self, address: String, data: ChallengeData);

    fn cache_remove(&self, address: &String);

    fn new_cache() -> Cache<String, ChallengeData>
    where
        Self: Sized;
}
