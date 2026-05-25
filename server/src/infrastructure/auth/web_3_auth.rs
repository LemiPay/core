use crate::application::auth::traits::challenge_cache::{ChallengeCacheTrait, Web3AuthCacheTrait};
use crate::application::auth::traits::web3_auth::Web3AuthTrait;
use alloy::primitives::{Address, Bytes, eip191_hash_message};
use alloy::providers::ProviderBuilder;
use async_trait::async_trait;
use erc6492::verify_signature;
use moka::sync::Cache;
use std::env;
use std::time::Duration;
use uuid::Uuid;

#[derive(Clone)]
pub struct ChallengeData {
    pub nonce: String,
    pub issued_at: String,
}

pub struct Web3Auth {
    rpc_url: String,
    cache: Cache<String, ChallengeData>,
}

impl Web3Auth {
    pub fn new() -> Self {
        match env::var("ALCHEMY_RPC_URL") {
            Ok(url) => Self {
                rpc_url: url,
                cache: Web3Auth::new_cache(),
            },
            Err(_) => panic!("ALCHEMY_RPC_URL environment variable not set"),
        }
    }
}
#[async_trait]
impl Web3AuthTrait for Web3Auth {
    fn generate_nonce(&self) -> String {
        Uuid::new_v4().to_string()
    }

    fn generate_issued_at(&self) -> String {
        chrono::Utc::now().to_rfc3339()
    }

    fn generate_message(&self, address: &Address, nonce: &String, issued_at: &String) -> String {
        format!(
            "lemi.pay quiere iniciar sesión con tu cuenta Ethereum:\n\
        {}\n\n\
        Bienvenido a LemiPay.\n\n\
        URI: https://localhost:5173\n\
        Version: 1\n\
        Chain ID: 1\n\
        Nonce: {}\n\
        Issued At: {}",
            address, nonce, issued_at
        )
    }

    async fn validate_signature_rpc(
        &self,
        address: String,
        signature_hex: String,
        nonce: String,
        issued_at: String,
    ) -> bool {
        let nonce = nonce.trim().to_string();

        let address_trim = match address.trim().parse::<Address>() {
            Ok(a) => a,
            Err(_) => return false,
        };

        let signature_trim = match signature_hex.trim().parse::<Bytes>() {
            Ok(s) => s,
            Err(_) => return false,
        };

        let message = eip191_hash_message(self.generate_message(&address_trim, &nonce, &issued_at));

        let rpc_url = match self.rpc_url.parse() {
            Ok(url) => url,
            Err(_) => return false,
        };

        let provider = ProviderBuilder::new().connect_http(rpc_url);

        match verify_signature(signature_trim, address_trim, message, &provider).await {
            Ok(verification) => verification.is_valid(),
            Err(_) => false,
        }
    }
}

#[async_trait]
impl ChallengeCacheTrait for Web3Auth {
    fn cache_get(&self, address: &String) -> Option<ChallengeData> {
        self.cache.get(address)
    }

    fn cache_insert(&self, address: String, data: ChallengeData) {
        self.cache.insert(address, data);
    }

    fn cache_remove(&self, address: &String) {
        self.cache.invalidate(address);
    }

    fn new_cache() -> Cache<String, ChallengeData> {
        let cache = Cache::builder()
            .max_capacity(10_000)
            .time_to_live(Duration::from_secs(900))
            .build();

        cache
    }
}

impl Web3AuthCacheTrait for Web3Auth {}
