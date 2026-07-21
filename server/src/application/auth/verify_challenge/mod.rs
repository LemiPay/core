use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

use crate::application::auth::new_user::NewUser;
use crate::application::auth::traits::challenge_cache::Web3AuthCacheTrait;
use crate::application::auth::traits::repository::AuthRepository;
use crate::application::auth::traits::token_service::TokenService;
use crate::application::auth::verify_challenge::dto::{VerificationInput, VerificationOutput};
use crate::application::notifications::repository::NotificationRepository;
use crate::application::treasury::traits::user_wallet_repo::UserWalletRepository;
use crate::application::users::traits::repository::UserRepository;
use crate::domain::treasury::{CurrencyId, Money, UserWallet, UserWalletId};
use crate::domain::user::{Email, UserId};
use crate::infrastructure::auth::jwt_service::JwtService;
use crate::infrastructure::auth::web_3_auth::ChallengeData;
use crate::interfaces::http::error::AppError;

pub mod dto;

pub struct VerifyChallengeUseCase {
    pub web3_service: Arc<dyn Web3AuthCacheTrait>,
    pub user_repository: Arc<dyn UserRepository>,
    pub user_wallet_repository: Arc<dyn UserWalletRepository>,
    jwt_service: Arc<JwtService>,
    pub auth_repository: Arc<dyn AuthRepository>,
    pub notification_repo: Arc<dyn NotificationRepository>,
}

impl VerifyChallengeUseCase {
    pub fn new(
        web3_service: Arc<dyn Web3AuthCacheTrait>,
        user_repository: Arc<dyn UserRepository>,
        user_wallet_repository: Arc<dyn UserWalletRepository>,
        jwt_service: Arc<JwtService>,
        auth_repository: Arc<dyn AuthRepository>,
        notification_repo: Arc<dyn NotificationRepository>,
    ) -> Self {
        Self {
            web3_service,
            user_repository,
            user_wallet_repository,
            jwt_service,
            auth_repository,
            notification_repo,
        }
    }

    pub async fn verify_challenge(
        &self,
        input: VerificationInput,
    ) -> Result<VerificationOutput, AppError> {
        let parsed_address: alloy::primitives::Address = input
            .address
            .trim()
            .parse()
            .map_err(|_| AppError::BadRequest("Dirección Ethereum inválida".into()))?;
        let address = format!("{parsed_address:#x}").to_lowercase();

        let issued_at = Self::resolve_issued_at(
            self.web3_service.cache_get(&address),
            &input.nonce,
            input.issued_at.as_deref(),
        )?;

        let is_valid = self
            .web3_service
            .validate_signature_rpc(
                address.clone(),
                input.signature,
                input.nonce.clone(),
                issued_at,
            )
            .await;

        if !is_valid {
            return Err(AppError::Forbidden("Firma criptográfica inválida".into()));
        }

        // Consumir challenge en esta instancia (anti-replay local).
        self.web3_service.cache_remove(&address);

        let email = match input
            .email
            .as_ref()
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
        {
            Some(value) => Some(
                Email::new(value.to_string())
                    .map_err(|_| AppError::BadRequest("Email inválido".into()))?,
            ),
            None => None,
        };

        let name = input
            .name
            .as_ref()
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
            .map(|value| value.to_string());

        let id = match email {
            Some(mail) => {
                let find_user = self
                    .user_repository
                    .find_by_email(&mail)
                    .map_err(|_| AppError::Internal)?;

                match find_user {
                    Some(user) => {
                        let user_id = UserId(user.id.clone());
                        let owner = self
                            .user_wallet_repository
                            .find_owner_of_address(&address)
                            .map_err(|_| AppError::Internal)?;

                        match owner {
                            Some(owner_id) if owner_id == user_id => {
                                _ = self.handle_known_user(user_id.clone(), address);
                                user_id
                            }
                            Some(_) => {
                                return Err(AppError::BadRequest(
                                    "La wallet ya está asociada a otra cuenta".into(),
                                ));
                            }
                            None => {
                                if input.allow_linking {
                                    _ = self.handle_known_user(user_id.clone(), address);
                                    user_id
                                } else {
                                    return Err(AppError::BadRequest(
                                        "Ese email ya tiene una cuenta. Iniciá sesión con contraseña o Google y vinculá la wallet desde tu perfil.".into(),
                                    ));
                                }
                            }
                        }
                    }
                    None => self.handle_new_user(mail, address, name)?,
                }
            }
            None => {
                let owner = self
                    .user_wallet_repository
                    .find_owner_of_address(&address)
                    .map_err(|_| AppError::Internal)?;

                match owner {
                    Some(user_id) => {
                        _ = self.handle_known_user(user_id.clone(), address);
                        user_id
                    }
                    None => {
                        return Err(AppError::BadRequest(
                            "Email requerido para asociar la wallet".into(),
                        ));
                    }
                }
            }
        };

        let token = self
            .jwt_service
            .generate(id.clone())
            .map_err(|_| AppError::Internal)?;

        Ok(VerificationOutput {
            token: token.0,
            user_id: id.to_string(),
        })
    }

    /// Resuelve `issued_at` del challenge:
    /// 1) cache in-memory con el mismo nonce (mismo proceso)
    /// 2) fallback al `issued_at` que reenvía el cliente (multi-réplica / race)
    fn resolve_issued_at(
        cached: Option<ChallengeData>,
        nonce: &str,
        client_issued_at: Option<&str>,
    ) -> Result<String, AppError> {
        if let Some(data) = cached {
            if data.nonce == nonce {
                return Ok(data.issued_at);
            }
            // Race o multi-request: el cache tiene otro challenge.
            // Si el cliente manda issued_at fresco del challenge que firmó, usarlo.
            if let Some(issued_at) = client_issued_at.map(str::trim).filter(|v| !v.is_empty()) {
                if Self::is_issued_at_fresh(issued_at) {
                    return Ok(issued_at.to_string());
                }
            }
            return Err(AppError::Forbidden("Nonce inválido".into()));
        }

        let Some(issued_at) = client_issued_at.map(str::trim).filter(|v| !v.is_empty()) else {
            return Err(AppError::Forbidden(
                "Sesión expirada o desafío no solicitado".into(),
            ));
        };

        if !Self::is_issued_at_fresh(issued_at) {
            return Err(AppError::Forbidden(
                "Sesión expirada o desafío no solicitado".into(),
            ));
        }

        Ok(issued_at.to_string())
    }

    fn is_issued_at_fresh(issued_at: &str) -> bool {
        let Ok(parsed) = chrono::DateTime::parse_from_rfc3339(issued_at) else {
            return false;
        };
        let issued = parsed.with_timezone(&chrono::Utc);
        let now = chrono::Utc::now();
        let age = now.signed_duration_since(issued);
        // 15 min de vida del challenge + 1 min de skew de reloj
        age <= chrono::Duration::seconds(900) && age >= chrono::Duration::seconds(-60)
    }

    fn handle_new_user(
        &self,
        mail: Email,
        addr: String,
        name: Option<String>,
    ) -> Result<UserId, AppError> {
        let resolved_name = name.unwrap_or_else(|| addr.clone());
        let new_user = NewUser {
            email: mail.0,
            password: None,
            name: resolved_name,
        };

        let saved_user = self
            .auth_repository
            .save(&new_user)
            .map_err(|_| AppError::Internal)?;

        let real_user_id = saved_user.user.id;

        // Seed default notification preferences for users created via wallet auth / first link
        let _ = self
            .notification_repo
            .initialize_defaults_for_user(real_user_id.clone());

        let user_wallet = UserWallet {
            id: UserWalletId(Uuid::new_v4()),
            address: addr,
            user_id: real_user_id.clone(),
            balance: Money {
                amount: Default::default(),
                currency: CurrencyId(
                    Uuid::from_str("33de6c7c-62a2-4182-813a-9005183be70d")
                        .map_err(|_| AppError::Internal)?,
                ),
            },
        };

        self.user_wallet_repository
            .save(&user_wallet)
            .map_err(|_| AppError::Internal)?;

        Ok(real_user_id)
    }

    fn handle_known_user(&self, user_id: UserId, addr: String) -> Result<UserId, AppError> {
        let usdc_currency = CurrencyId(
            Uuid::from_str("33de6c7c-62a2-4182-813a-9005183be70d")
                .map_err(|_| AppError::Internal)?,
        );
        let user_wallet = self
            .user_wallet_repository
            .find_by_address_and_currency(&addr, usdc_currency.clone())
            .map_err(|_| AppError::Internal)?;

        if user_wallet.is_some() {
            return Ok(user_id);
        }

        let wallet = UserWallet {
            id: UserWalletId(Uuid::new_v4()),
            address: addr,
            user_id,
            balance: Money {
                amount: Default::default(),
                currency: CurrencyId(
                    Uuid::from_str("33de6c7c-62a2-4182-813a-9005183be70d")
                        .map_err(|_| AppError::Internal)?,
                ),
            },
        };

        self.user_wallet_repository
            .save(&wallet)
            .map_err(|_| AppError::Internal)?;

        Ok(user_id)
    }
}

#[cfg(test)]
mod tests;
