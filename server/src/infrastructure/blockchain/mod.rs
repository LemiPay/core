use alloy::primitives::{Address, B256, Bytes};
use alloy::providers::ProviderBuilder;
use alloy::rpc::types::{Filter, Log};
use async_trait::async_trait;

use crate::domain::treasury::CurrencyAddress;

use crate::infrastructure::blockchain::ethereum_service::TokenAddedEvent;
use crate::infrastructure::blockchain::{
    contracts::lemipay_vault::LemiPayVault, error::BlockchainError,
};

pub mod contracts;
pub mod error;
pub mod ethereum_service;

#[async_trait]
pub trait BlockchainService: Send + Sync {
    async fn verify_signature(&self, sig: Bytes, address: Address, msg: B256) -> bool;

    async fn get_supported_tokens(
        &self,
        currency_addr: CurrencyAddress,
    ) -> Result<LemiPayVault::supportedTokensReturn, BlockchainError>;

    async fn get_events(
        &self,
        _from_block: u64,
        _to_block: u64,
    ) -> Result<Vec<TokenAddedEvent>, BlockchainError>;
}
