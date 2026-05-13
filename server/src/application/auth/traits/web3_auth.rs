use async_trait::async_trait;

#[async_trait]
pub trait Web3AuthTrait: Send + Sync {
    fn generate_nonce(&self) -> String;

    fn validate_signature(
        &self,
        email: String,
        address: String,
        signature: String,
        nonce: String,
    ) -> bool;

    async fn validate_signature_eip1271(
        &self,
        email: String,
        address: String,
        signature_hex: String,
        nonce: String,
    ) -> bool;
}
