use crate::application::auth::traits::web3_auth::Web3AuthTrait;
use alloy::primitives::{Address, Bytes, Signature};
use alloy::providers::ProviderBuilder;
use alloy::sol;
use async_trait::async_trait;
use std::env;
use std::str::FromStr;
use uuid::Uuid;

// 1. Definimos la interfaz del Smart Contract para EIP-1271
sol! {
    #[sol(rpc)]
    interface IERC1271 {
        function isValidSignature(bytes32 hash, bytes signature) external view returns (bytes4 magicValue);
    }
}

// Valor mágico que devuelve el contrato si la firma es correcta
const EIP1271_MAGIC_VALUE: [u8; 4] = [0x16, 0x26, 0xba, 0x7e];

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

    async fn validate_signature_eip1271(
        &self,
        email: String,
        address: String,
        signature_hex: String,
        nonce: String,
    ) -> bool {
        let email = email.trim().to_lowercase();
        let nonce = nonce.trim();
        let address_trim = address.trim();
        let signature_trim = signature_hex.trim();

        let message = format!(
            "Bienvenido a LemiPay.\n\n\
            Al firmar este mensaje, confirmas que eres el dueño de esta cuenta.\n\n\
            Email: {}\n\
            Nonce: {}",
            email, nonce
        );

        let expected_addr = match Address::from_str(address_trim) {
            Ok(a) => a,
            Err(_) => return false,
        };

        // --- INTENTO 1: Verificación tradicional (EOA / MetaMask) ---
        if let Ok(sig) = Signature::from_str(signature_trim) {
            if let Ok(recovered_addr) = sig.recover_address_from_msg(&message) {
                if recovered_addr == expected_addr {
                    return true; // Es una wallet normal y validó perfecto
                }
            }
        }

        let alchemy_url_string =
            env::var("ALCHEMY_RPC_URL").expect("ALCHEMY_URL must be set in .env");

        let rpc_url = match alchemy_url_string.parse() {
            Ok(url) => url,
            Err(e) => {
                println!("ERROR PARSEANDO URL RPC: {:?}", e);
                return false;
            }
        };

        let provider = ProviderBuilder::new().on_http(rpc_url);

        let contract = IERC1271::new(expected_addr, provider);

        let formatted_msg = format!("\x19Ethereum Signed Message:\n{}{}", message.len(), message);
        let message_hash = alloy::primitives::keccak256(formatted_msg);

        let signature_bytes = match signature_trim.parse::<Bytes>() {
            Ok(b) => b,
            Err(e) => {
                println!("ERROR PARSEANDO FIRMA A BYTES: {:?}", e);
                return false;
            }
        };

        // Consultamos a la blockchain
        match contract
            .isValidSignature(message_hash, signature_bytes)
            .call()
            .await
        {
            Ok(result) => result.magicValue == EIP1271_MAGIC_VALUE,
            Err(e) => {
                println!(
                    "⚠️ EIP-1271 Falló (¿Contrato no desplegado o red incorrecta?). Address: {}",
                    expected_addr
                );
                println!("Detalle técnico: {:?}", e);
                false
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::signers::SignerSync;
    use alloy::signers::local::PrivateKeySigner;
    #[test]
    fn test_validate_signature() {
        let web3_auth = Web3Auth::new();

        let signer = PrivateKeySigner::random();
        let address = signer.address().to_string();
        let email = "facu@mail".to_string();
        let nonce = "1fb010a6-c8a5-40d5-b5a1-e9cecfb98b3f".to_string();

        let message = format!(
            "Bienvenido a LemiPay.\n\n\
            Al firmar este mensaje, confirmas que eres el dueño de esta cuenta.\n\n\
            Email: {}\n\
            Nonce: {}",
            email, nonce
        );

        let signature_obj = signer
            .sign_message_sync(message.as_bytes())
            .expect("Falló la firma");
        let signature_hex = format!("0x{}", alloy::hex::encode(signature_obj.as_bytes()));
        // 4. Validamos
        let is_valid = web3_auth.validate_signature(email, address, signature_hex, nonce);

        assert!(is_valid, "La firma debería ser válida");
    }
    #[test]
    fn test_validate_signature_failed_different_address() {
        let web3_auth = Web3Auth::new();

        let real_signer = PrivateKeySigner::random();
        let impostor_signer = PrivateKeySigner::random();

        let impostor_address = impostor_signer.address().to_string();

        let email = "facu@mail".to_string();
        let nonce = "1fb010a6-c8a5-40d5-b5a1-e9cecfb98b3f".to_string();

        let message = format!(
            "Bienvenido a LemiPay.\n\n\
        Al firmar este mensaje, confirmas que eres el dueño de esta cuenta.\n\n\
        Email: {}\n\
        Nonce: {}",
            email, nonce
        );

        let signature_obj = real_signer
            .sign_message_sync(message.as_bytes())
            .expect("Falló la firma");
        let signature_hex = format!("0x{}", alloy::hex::encode(signature_obj.as_bytes()));

        let is_valid = web3_auth.validate_signature(email, impostor_address, signature_hex, nonce);

        assert!(
            !is_valid,
            "La validación debería fallar porque la dirección no coincide con quien firmó"
        );
    }
    #[test]
    fn test_validate_signature_corrupted_hex() {
        let web3_auth = Web3Auth::new();
        let signer = PrivateKeySigner::random();
        let address = signer.address().to_string();
        let email = "facu@mail".to_string();
        let nonce = "1fb010a6-c8a5-40d5-b5a1-e9cecfb98b3f".to_string();

        let message = format!(
            "Bienvenido a LemiPay.\n\n\
        Al firmar este mensaje, confirmas que eres el dueño de esta cuenta.\n\n\
        Email: {}\n\
        Nonce: {}",
            email, nonce
        );

        let signature_obj = signer
            .sign_message_sync(message.as_bytes())
            .expect("Falló la firma");
        let mut signature_hex = format!("0x{}", alloy::hex::encode(signature_obj.as_bytes()));

        if signature_hex.ends_with('a') {
            signature_hex.replace_range(signature_hex.len() - 1.., "b");
        } else {
            signature_hex.replace_range(signature_hex.len() - 1.., "a");
        }

        let is_valid = web3_auth.validate_signature(email, address, signature_hex, nonce);

        assert!(
            !is_valid,
            "La firma corrompida no debería pasar la validación"
        );
    }
}
