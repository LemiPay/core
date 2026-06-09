use alloy::primitives::{Address, B256, Bytes};
use async_trait::async_trait;

pub mod ethereum_service;

#[async_trait]
pub trait BlockchainService: Send + Sync {
    async fn verify_signature(&self, sig: Bytes, address: Address, msg: B256) -> bool;
}
