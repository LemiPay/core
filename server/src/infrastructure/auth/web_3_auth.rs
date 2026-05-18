use crate::application::auth::traits::web3_auth::Web3AuthTrait;
use alloy::primitives::{Address, Bytes, Signature, eip191_hash_message};
use alloy::providers::ProviderBuilder;
use alloy::sol;
use async_trait::async_trait;
use erc6492::verify_signature;
use std::env;
use std::str::FromStr;
use uuid::Uuid;
sol! {
    #[sol(rpc)]
    interface IERC1271 {
        function isValidSignature(bytes32 hash, bytes signature) external view returns (bytes4 magicValue);
    }
}

pub struct Web3Auth {}

impl Web3Auth {
    pub fn new() -> Self {
        Self {}
    }
}
#[async_trait]
impl Web3AuthTrait for Web3Auth {
    fn generate_nonce(&self) -> String {
        Uuid::new_v4().to_string()
    }

    fn validate_signature(
        &self,
        email: String,
        address: String,
        signature: String,
        nonce: String,
    ) -> bool {
        let message = format!(
            "Bienvenido a LemiPay.\n\n\
            Al firmar este mensaje, confirmas que eres el dueño de esta cuenta.\n\n\
            Email: {}\n\
            Nonce: {}",
            email, nonce
        );
        let sig = match Signature::from_str(&signature) {
            Ok(s) => s,
            Err(_) => return false,
        };

        let expected_addr = match Address::from_str(&address) {
            Ok(a) => a,
            Err(_) => return false,
        };

        match sig.recover_address_from_msg(message) {
            Ok(recovered_addr) => recovered_addr == expected_addr,
            Err(_) => false,
        }
    }

    async fn validate_signature_rpc(
        &self,
        email: String,
        address: String,
        signature_hex: String,
        nonce: String,
    ) -> bool {
        let email = email.trim().to_lowercase();
        let nonce = nonce.trim();

        let address_trim = match address.trim().parse::<Address>() {
            Ok(a) => a,
            Err(_) => return false,
        };

        let signature_trim = match signature_hex.trim().parse::<Bytes>() {
            Ok(s) => s,
            Err(_) => return false,
        };

        let message = eip191_hash_message(format!(
            "Bienvenido a LemiPay.\n\n\
        Al firmar este mensaje, confirmas que eres el dueño de esta cuenta.\n\n\
        Email: {}\n\
        Nonce: {}",
            email, nonce
        ));

        let rpc_url = match env::var("ALCHEMY_RPC_URL") {
            Ok(url) => url,
            Err(_) => return false,
        };

        let rpc_url = match rpc_url.parse() {
            Ok(url) => url,
            Err(_) => return false,
        };

        let provider = ProviderBuilder::new().connect_http(rpc_url);

        match verify_signature(signature_trim, address_trim, message, &provider).await {
            Ok(verification) => verification.is_valid(),
            Err(_) => false,
        }
    }
}
