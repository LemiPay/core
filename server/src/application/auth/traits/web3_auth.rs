
pub trait Web3AuthTrait: Send + Sync {
    fn generate_nonce(&self) -> String;
}