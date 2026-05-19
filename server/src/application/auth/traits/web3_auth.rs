use async_trait::async_trait;

#[async_trait]
pub trait Web3AuthTrait: Send + Sync {
    fn generate_nonce(&self) -> String;

    fn generate_message(&self, email: String, nonce: String) -> String;

    async fn validate_signature_rpc(
        &self,
        email: String,
        address: String,
        signature_hex: String,
        nonce: String,
    ) -> bool;
}
