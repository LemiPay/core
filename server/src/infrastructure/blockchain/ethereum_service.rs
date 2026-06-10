use crate::domain::treasury::CurrencyAddress;
use crate::infrastructure::blockchain::{
    BlockchainService, ContractEvent, contracts::lemipay_vault::LemiPayVault,
    error::BlockchainError, event_decoder::try_decode_event,
};
use alloy::primitives::{Address, B256, Bytes, U256};
use alloy::providers::{Provider, ProviderBuilder};
use alloy::rpc::types::Filter;
use alloy::signers::local::PrivateKeySigner;
use async_trait::async_trait;
use erc6492;
use std::env;

#[derive(Clone)]
pub struct EthereumService {
    rpc_url: String,
    vault_address: Address,
}

impl EthereumService {
    pub fn new() -> Self {
        dotenvy::dotenv().ok();
        dotenvy::from_filename("../.env").ok();

        let rpc_url = env::var("RPC_URL").expect("RPC_URL environment variable not set");

        let vault_address_str =
            env::var("VAULT_ADDRESS").expect("VAULT_ADDRESS environment variable not set");

        Self {
            rpc_url,
            vault_address: vault_address_str.parse().expect("Invalid VAULT_ADDRESS"),
        }
    }
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
            .map_err(|e| BlockchainError::BlockchainService(e.to_string()))?;

        Ok(supported_tokens_response)
    }

    async fn get_block_number(&self) -> Result<u64, BlockchainError> {
        let provider = ProviderBuilder::new().connect_http(self.rpc_url.parse().unwrap());
        let block_number = provider
            .get_block_number()
            .await
            .map_err(|e| BlockchainError::RpcError(e.to_string()))?;
        Ok(block_number)
    }

    async fn get_events(
        &self,
        from_block: u64,
        to_block: u64,
    ) -> Result<Vec<ContractEvent>, BlockchainError> {
        let provider = ProviderBuilder::new().connect_http(self.rpc_url.parse().unwrap());

        let filter = Filter::new()
            .from_block(from_block)
            .to_block(to_block)
            .address(self.vault_address);

        let logs = provider
            .get_logs(&filter)
            .await
            .map_err(|e| BlockchainError::RpcError(e.to_string()))?;

        let events: Vec<ContractEvent> = logs.into_iter().filter_map(try_decode_event).collect();

        Ok(events)
    }

    async fn withdraw(
        &self,
        receiver: Address,
        wallet_address: B256,
        token: Address,
        amount: U256,
    ) -> Result<String, BlockchainError> {
        let private_key_str = env::var("PRIVATE_KEY")
            .map_err(|_| BlockchainError::BlockchainService("PRIVATE_KEY not set".into()))?;

        let signer: PrivateKeySigner = private_key_str
            .parse()
            .map_err(|e| BlockchainError::BlockchainService(format!("Invalid PRIVATE_KEY: {e}")))?;

        let provider = ProviderBuilder::new()
            .wallet(signer)
            .connect_http(self.rpc_url.parse().unwrap());

        let vault = LemiPayVault::new(self.vault_address, &provider);

        let pending = vault
            .withdraw(receiver, wallet_address, token, amount)
            .send()
            .await
            .map_err(|e| {
                BlockchainError::BlockchainService(format!("withdraw send failed: {e}"))
            })?;

        let receipt = pending.get_receipt().await.map_err(|e| {
            BlockchainError::BlockchainService(format!("failed to get receipt: {e}"))
        })?;

        Ok(format!("{:?}", receipt.transaction_hash))
    }
}
