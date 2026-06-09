use crate::infrastructure::blockchain::BlockchainService;
use alloy::primitives::{Address, B256, Bytes};
use alloy::providers::ProviderBuilder;
use async_trait::async_trait;
use erc6492::verify_signature;
use std::env;

pub struct EthereumService {
    rpc_url: String,
}

impl EthereumService {
    pub fn new() -> Self {
        let rpc_url = env::var("RPC_URL").expect("RPC_URL environment variable not set");

        Self { rpc_url }
    }
}

#[async_trait]
impl BlockchainService for EthereumService {
    async fn verify_signature(&self, sig: Bytes, address: Address, msg: B256) -> bool {
        let provider = ProviderBuilder::new().connect_http(self.rpc_url.parse().unwrap());

        match verify_signature(sig, address, msg, &provider).await {
            Ok(verification) => verification.is_valid(),
            Err(_) => false,
        }
    }
}
