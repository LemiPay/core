use alloy::primitives::{Address, B256, Bytes};
use async_trait::async_trait;

use crate::domain::treasury::CurrencyAddress;
use crate::infrastructure::blockchain::{
    contracts::lemipay_vault::LemiPayVault, error::BlockchainError,
};

pub mod contracts;
pub mod error;
pub mod ethereum_service;
mod event_decoder;
pub mod events;

pub use events::*;

#[async_trait]
pub trait BlockchainService: Send + Sync {
    async fn verify_signature(&self, sig: Bytes, address: Address, msg: B256) -> bool;

    async fn get_supported_tokens(
        &self,
        currency_addr: CurrencyAddress,
    ) -> Result<LemiPayVault::supportedTokensReturn, BlockchainError>;

    async fn get_block_number(&self) -> Result<u64, BlockchainError>;

    async fn get_events(
        &self,
        from_block: u64,
        to_block: u64,
    ) -> Result<Vec<ContractEvent>, BlockchainError>;
}
