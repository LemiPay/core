use alloy::primitives::Address;
use async_trait::async_trait;
#[async_trait]

pub trait Web3AuthTrait: Send + Sync {
    fn generate_nonce(&self) -> String;
    fn generate_issued_at(&self) -> String;

    fn generate_message(&self, address: &Address, nonce: &String, issued_at: &String) -> String;

    async fn validate_signature_rpc(
        &self,
        address: String,
        signature_hex: String,
        nonce: String,
        issued_at: String,
    ) -> bool;
}
