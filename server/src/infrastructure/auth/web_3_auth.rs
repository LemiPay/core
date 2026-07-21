use crate::application::auth::traits::challenge_cache::{ChallengeCacheTrait, Web3AuthCacheTrait};
use crate::application::auth::traits::web3_auth::Web3AuthTrait;
use crate::infrastructure::blockchain::BlockchainService;
use crate::infrastructure::blockchain::ethereum_service::EthereumService;
use alloy::primitives::{Address, Bytes, eip191_hash_message};
use async_trait::async_trait;
use moka::sync::Cache;
use std::sync::Arc;
use std::time::Duration;
use uuid::Uuid;

#[derive(Clone)]
pub struct ChallengeData {
    pub nonce: String,
    pub issued_at: String,
}

pub struct Web3Auth {
    blockchain_service: Arc<dyn BlockchainService>,
    cache: Cache<String, ChallengeData>,
    /// Origen de la app mostrado en el mensaje a firmar (sin trailing slash).
    app_uri: String,
}

/// URI del frontend para mensajes SIWE-like.
/// Variable de entorno: `FRONTEND_URL`. Default: `https://lemipay.app`.
fn resolve_app_uri() -> String {
    std::env::var("FRONTEND_URL")
        .ok()
        .map(|value| value.trim().trim_end_matches('/').to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| "https://lemipay.app".to_string())
}

impl Web3Auth {
    pub fn new() -> Self {
        dotenvy::dotenv().ok();
        dotenvy::from_filename("../.env").ok();

        Self {
            blockchain_service: Arc::new(EthereumService::new()),
            cache: Web3Auth::new_cache(),
            app_uri: resolve_app_uri(),
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
        // Sepolia (11155111): debe coincidir con la red configurada en el cliente Reown/AppKit.
        // URI: `FRONTEND_URL` o default `https://lemipay.app`.
        format!(
            "lemipay.app quiere iniciar sesión con tu cuenta Ethereum:\n\
        {}\n\n\
        Bienvenido a LemiPay.\n\n\
        URI: {}\n\
        Version: 1\n\
        Chain ID: 11155111\n\
        Nonce: {}\n\
        Issued At: {}",
            address, self.app_uri, nonce, issued_at
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

        self.blockchain_service
            .verify_signature(signature_trim, address_trim, message)
            .await
    }
}

/// Clave canónica para el cache de challenges (checksum-insensitive).
fn challenge_cache_key(address: &str) -> String {
    match address.trim().parse::<Address>() {
        Ok(addr) => format!("{addr:#x}").to_lowercase(),
        Err(_) => address.trim().to_lowercase(),
    }
}

#[async_trait]
impl ChallengeCacheTrait for Web3Auth {
    fn cache_get(&self, address: &String) -> Option<ChallengeData> {
        self.cache.get(&challenge_cache_key(address))
    }

    fn cache_insert(&self, address: String, data: ChallengeData) {
        self.cache.insert(challenge_cache_key(&address), data);
    }

    fn cache_remove(&self, address: &String) {
        self.cache.invalidate(&challenge_cache_key(address));
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
