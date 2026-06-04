use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

use crate::application::auth::new_user::NewUser;
use crate::application::auth::traits::challenge_cache::Web3AuthCacheTrait;
use crate::application::auth::traits::repository::AuthRepository;
use crate::application::auth::traits::token_service::TokenService;
use crate::application::auth::verify_challenge::dto::{VerificationInput, VerificationOutput};
use crate::application::treasury::traits::user_wallet_repo::UserWalletRepository;
use crate::application::users::traits::repository::UserRepository;
use crate::domain::treasury::{CurrencyId, Money, UserWallet, UserWalletId};
use crate::domain::user::{Email, UserId};
use crate::infrastructure::auth::jwt_service::JwtService;
use crate::interfaces::http::error::AppError;

pub mod dto;

pub struct VerifyChallengeUseCase {
    pub web3_service: Arc<dyn Web3AuthCacheTrait>,
    pub user_repository: Arc<dyn UserRepository>,
    pub user_wallet_repository: Arc<dyn UserWalletRepository>,
    jwt_service: Arc<JwtService>,
    pub auth_repository: Arc<dyn AuthRepository>,
}

impl VerifyChallengeUseCase {
    pub fn new(
        web3_service: Arc<dyn Web3AuthCacheTrait>,
        user_repository: Arc<dyn UserRepository>,
        user_wallet_repository: Arc<dyn UserWalletRepository>,
        jwt_service: Arc<JwtService>,
        auth_repository: Arc<dyn AuthRepository>,
    ) -> Self {
        Self {
            web3_service,
            user_repository,
            user_wallet_repository,
            jwt_service,
            auth_repository,
        }
    }

    pub async fn verify_challenge(
        &self,
        input: VerificationInput,
    ) -> Result<VerificationOutput, AppError> {
        let stored_data = self
            .web3_service
            .cache_get(&input.address.trim().to_string());

        let Some(data) = stored_data else {
            return Err(AppError::Forbidden(
                "Sesión expirada o desafío no solicitado".into(),
            ));
        };

        if data.nonce != input.nonce {
            return Err(AppError::Forbidden("Nonce inválido".into()));
        }

        let is_valid = self
            .web3_service
            .validate_signature_rpc(
                input.address.clone(),
                input.signature,
                data.nonce,
                data.issued_at,
            )
            .await;

        if !is_valid {
            return Err(AppError::Forbidden("Firma criptográfica inválida".into()));
        }

        self.web3_service
            .cache_remove(&input.address.trim().to_string());

        let address = input.address.trim().to_string();

        let email = input
            .email
            .as_ref()
            .map(|value| value.trim())
            .filter(|value| !value.is_empty())
            .map(|value| Email(value.to_lowercase()));

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
                                        "Email ya está asociado a una cuenta".into(),
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
