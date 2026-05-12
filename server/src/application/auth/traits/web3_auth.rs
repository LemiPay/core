pub trait Web3AuthTrait: Send + Sync {
    fn generate_nonce(&self) -> String;

    fn validate_signature(
        &self,
        email: String,
        address: String,
        signature: String,
        nonce: String,
    ) -> bool;
}
