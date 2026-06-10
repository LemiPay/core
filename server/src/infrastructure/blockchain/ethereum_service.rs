use crate::domain::treasury::CurrencyAddress;
use crate::infrastructure::blockchain::contracts::lemipay_vault::LemiPayVault::TokenAdded;
use crate::infrastructure::blockchain::{
    BlockchainService, contracts::lemipay_vault::LemiPayVault, error::BlockchainError,
};
use alloy::primitives::{Address, B256, Bytes};
use alloy::providers::{Provider, ProviderBuilder};
use alloy::rpc::types::{Filter, Log};
use async_trait::async_trait;
use erc6492;
use std::env;

pub struct EthereumService {
    rpc_url: String,
    vault_address: Address,
}

impl EthereumService {
    pub fn new() -> Self {
        dotenvy::dotenv().ok(); // intenta cargar .env del cwd
        dotenvy::from_filename("../.env").ok(); // fallback

        let rpc_url = env::var("RPC_URL").expect("RPC_URL environment variable not set");

        let vault_address_str =
            env::var("VAULT_ADDRESS").expect("VAULT_ADDRESS environment variable not set");

        Self {
            rpc_url,
            vault_address: vault_address_str.parse().expect("Invalid VAULT_ADDRESS"),
        }
    }
}

pub struct TokenAddedEvent {
    pub token: Address,
    pub currency_id: B256,
}

#[async_trait]
impl BlockchainService for EthereumService {
    async fn verify_signature(&self, sig: Bytes, address: Address, msg: B256) -> bool {
        let provider = ProviderBuilder::new().connect_http(self.rpc_url.parse().unwrap());

        match erc6492::verify_signature(sig, address, msg, &provider).await {
            Ok(verification) => verification.is_valid(),
            Err(_) => false,
        }
    }

    async fn get_supported_tokens(
        &self,
        currency_addr: CurrencyAddress,
    ) -> Result<LemiPayVault::supportedTokensReturn, BlockchainError> {
        let provider = ProviderBuilder::new().connect_http(self.rpc_url.parse().unwrap());
        let vault = LemiPayVault::new(self.vault_address, provider);

        let supported_tokens_response = vault
            .supportedTokens(*currency_addr.as_address())
            .call()
            .await
            .map_err(|_| BlockchainError::BlockchainService)?;

        Ok(supported_tokens_response)
    }

    async fn get_events(
        &self,
        _from_block: u64,
        _to_block: u64,
    ) -> Result<Vec<TokenAddedEvent>, BlockchainError> {
        let provider = ProviderBuilder::new().connect_http(self.rpc_url.parse().unwrap());
        let vault = LemiPayVault::new(self.vault_address, provider);

        let events = vault
            .TokenAdded_filter()
            .from_block(0)
            .to_block(9)
            .query()
            .await
            .map_err(|e| {
                println!("{e:?}");
                BlockchainError::BlockchainService
            })?;

        for event in &events {
            println!("token = {:?}", event.0.token);
            println!("currency_id = {:?}", event.0.currencyId);
        }

        let events = events
            .into_iter()
            .map(|event| TokenAddedEvent {
                token: event.0.token,
                currency_id: event.0.currencyId,
            })
            .collect();

        Ok(events)
    }
}
