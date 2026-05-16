use crate::application::auth::stored_user::StoredUser;
use crate::application::auth::traits::repository::AuthRepository;
use crate::application::auth::traits::token_service::TokenService;
use crate::application::auth::traits::web3_auth::Web3AuthTrait;
use crate::application::auth::verify_challenge::dto::{VerificationInput, VerificationOutput};
use crate::application::treasury::traits::user_wallet_repo::UserWalletRepository;
use crate::application::users::traits::repository::UserRepository;
use crate::domain::treasury::{CurrencyId, Money, UserWallet, UserWalletId};
use crate::domain::user::{Email, User, UserId, UserName};
use crate::infrastructure::auth::jwt_service::JwtService;
use crate::infrastructure::db::models::user::NewUserModel;
use crate::interfaces::http::error::AppError;
use moka::sync::Cache;
use std::ptr::null;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

pub mod dto;

pub struct VerifyChallengeUseCase {
    pub web3_service: Arc<dyn Web3AuthTrait>,
    pub user_repository: Arc<dyn UserRepository>,
    pub auth_repository: Arc<dyn AuthRepository>,
    pub user_wallet_repository: Arc<dyn UserWalletRepository>,
    nonce_cache: Cache<String, String>,
    jwt_service: Arc<JwtService>,
}

impl VerifyChallengeUseCase {
    pub fn new(
        web3_service: Arc<dyn Web3AuthTrait>,
        nonce_cache: Cache<String, String>,
        user_repository: Arc<dyn UserRepository>,
        user_wallet_repository: Arc<dyn UserWalletRepository>,
        auth_repository: Arc<dyn AuthRepository>,
        jwt_service: Arc<JwtService>,
    ) -> Self {
        Self {
            web3_service,
            user_repository,
            auth_repository,
            user_wallet_repository,
            nonce_cache,
            jwt_service,
        }
    }

    pub async fn verify_challenge(
        &self,
        input: VerificationInput,
    ) -> Result<VerificationOutput, AppError> {
        let stored_nonce = self.nonce_cache.get(&input.email);
        match stored_nonce {
            None => {
                return Err(AppError::Forbidden(
                    "Sesión expirada o desafío no solicitado".into(),
                ));
            }
            Some(n) if n != input.nonce => {
                return Err(AppError::Forbidden("Nonce inválido".into()));
            }
            _ => (),
        }

        let is_valid = self
            .web3_service
            .validate_signature_rpc(
                input.email.clone(),
                input.address.clone(),
                input.signature,
                input.nonce,
            )
            .await;

        if !is_valid {
            return Err(AppError::Forbidden("Firma criptográfica inválida".into()));
        }

        self.nonce_cache.remove(&input.email);
        let mail = Email(input.email.clone());

        let find_user = self
            .user_repository
            .find_by_email(&mail)
            .map_err(|_| AppError::Internal)?;

        let id = match find_user {
            Some(user) => {
                let user_id = UserId(user.id.clone());
                self.handle_known_user(user_id.clone(), mail, input.address);
                user_id
            }
            None => self.handle_new_user(mail, input.address)?,
        };

        // 3. Le agrego también el println! al JWT por si el error 500 venía de acá
        let token = self.jwt_service.generate(id.clone()).map_err(|e| {
            println!("❌ ERROR FATAL GENERANDO JWT: {:?}", e);
            AppError::Internal
        })?;

        Ok(VerificationOutput {
            token: token.0,
            user_id: id.to_string(),
        })
    }

    fn handle_new_user(&self, mail: Email, addr: String) -> Result<UserId, AppError> {
        let new_user = NewUserModel {
            email: mail.0,
            password: None,
            name: addr.to_string(),
        };

        let saved_user = self.auth_repository.save(&new_user).map_err(|e| {
            println!("❌ ERROR FATAL GUARDANDO USUARIO: {:?}", e);
            AppError::Internal
        })?;

        let real_user_id = saved_user.user.id;
        println!("✅ Usuario creado con ID real: {:?}", real_user_id);

        let user_wallet = UserWallet {
            id: UserWalletId(Uuid::new_v4()),
            address: addr,
            user_id: real_user_id.clone(),
            balance: Money {
                amount: Default::default(),
                currency: CurrencyId(
                    Uuid::from_str("33de6c7c-62a2-4182-813a-9005183be70d").map_err(|e| {
                        println!("❌ ERROR FATAL PARSEANDO UUID: {:?}", e);
                        AppError::Internal
                    })?,
                ),
            },
        };

        self.user_wallet_repository
            .save(&user_wallet)
            .map_err(|e| {
                println!("❌ ERROR FATAL GUARDANDO WALLET: {:?}", e);
                AppError::Internal
            })?;

        Ok(real_user_id)
    }

    fn handle_known_user(
        &self,
        user_id: UserId,
        mail: Email,
        addr: String,
    ) -> Result<UserId, AppError> {
        let usdc_currency = CurrencyId(
            Uuid::from_str("33de6c7c-62a2-4182-813a-9005183be70d").map_err(|e| {
                println!("❌ ERROR FATAL PARSEANDO UUID: {:?}", e);
                AppError::Internal
            })?,
        );
        let user_wallet = self
            .user_wallet_repository
            .find_by_address_and_currency(&addr, usdc_currency.clone())
            .map_err(|e| AppError::Internal)?;

        if user_wallet.is_some() {
            println!("Wallet ya existe",);
            return Ok(user_id);
        }

        let wallet = UserWallet {
            id: UserWalletId(Uuid::new_v4()),
            address: addr,
            user_id,
            balance: Money {
                amount: Default::default(),
                currency: CurrencyId(
                    Uuid::from_str("33de6c7c-62a2-4182-813a-9005183be70d").map_err(|e| {
                        println!("❌ ERROR FATAL PARSEANDO UUID: {:?}", e);
                        AppError::Internal
                    })?,
                ),
            },
        };

        self.user_wallet_repository.save(&wallet).map_err(|e| {
            println!("❌ ERROR FATAL GUARDANDO WALLET: {:?}", e);
            AppError::Internal
        })?;

        Ok(user_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::auth::verify_challenge;
    use crate::infrastructure::auth::web_3_auth::Web3Auth;
    use alloy::signers::SignerSync;
    use alloy::signers::local::PrivateKeySigner;

    #[test]
    fn simulate_frontend_payload() {
        let email = "facu@lemipay.com";
        let nonce = "e10c0174-d60d-4a77-b0b1-1137b1d38b65";

        let signer = PrivateKeySigner::random();
        let address = signer.address().to_string();

        let message = format!(
            "Bienvenido a LemiPay.\n\n\
            Al firmar este mensaje, confirmas que eres el dueño de esta cuenta.\n\n\
            Email: {}\n\
            Nonce: {}",
            email, nonce
        );

        let signature_obj = signer
            .sign_message_sync(message.as_bytes())
            .expect("Error firmando");
        let signature_hex = format!("0x{}", alloy::hex::encode(signature_obj.as_bytes()));

        println!("\n=== JSON para /verify-challenge ===");
        println!(
            r#"{{
  "email": "{}",
  "address": "{}",
  "signature": "{}",
  "nonce": "{}"
}}"#,
            email, address, signature_hex, nonce
        );
        println!("=======================================================\n");
    }
}
