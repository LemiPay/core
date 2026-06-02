pub mod dto;

use alloy::primitives::Address;
use std::sync::Arc;

use crate::application::auth::challenge::dto::{ChallengeInput, ChallengeOutput};
use crate::application::treasury::traits::user_wallet_repo::UserWalletRepository;
use crate::interfaces::http::error::AppError;

use crate::application::auth::traits::challenge_cache::Web3AuthCacheTrait;
use crate::infrastructure::auth::web_3_auth::ChallengeData;

pub struct ChallengeUseCase {
    pub web3_service: Arc<dyn Web3AuthCacheTrait>,
    pub user_wallet_repository: Arc<dyn UserWalletRepository>,
}

impl ChallengeUseCase {
    pub fn new(
        web3_service: Arc<dyn Web3AuthCacheTrait>,
        user_wallet_repository: Arc<dyn UserWalletRepository>,
    ) -> Self {
        Self {
            web3_service,
            user_wallet_repository,
        }
    }

    pub fn generate_challenge(&self, input: ChallengeInput) -> Result<ChallengeOutput, AppError> {
        let nonce = self.web3_service.generate_nonce();

        let issued_at = self.web3_service.generate_issued_at();

        let addr: Address = match input.address.trim().parse() {
            Ok(a) => a,
            Err(_) => return Err(AppError::BadRequest("Dirección Ethereum inválida".into())),
        };

        let message = self
            .web3_service
            .generate_message(&addr, &nonce, &issued_at);

        self.web3_service.cache_insert(
            input.address.clone(),
            ChallengeData {
                nonce: nonce.clone(),
                issued_at: issued_at.clone(),
            },
        );

        let is_linked = self
            .user_wallet_repository
            .find_owner_of_address(&input.address)
            .map_err(|_| AppError::Internal)?
            .is_some();

        Ok(ChallengeOutput {
            nonce,
            message,
            is_linked,
        })
    }
}

#[cfg(test)]
mod tests;
