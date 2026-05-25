pub mod dto;

use alloy::primitives::Address;
use std::sync::Arc;

use crate::application::auth::challenge::dto::{ChallengeInput, ChallengeOutput};
use crate::interfaces::http::error::AppError;

use crate::application::auth::traits::challenge_cache::Web3AuthCacheTrait;
use crate::infrastructure::auth::web_3_auth::ChallengeData;

pub struct ChallengeUseCase {
    pub web3_service: Arc<dyn Web3AuthCacheTrait>,
}

impl ChallengeUseCase {
    pub fn new(web3_service: Arc<dyn Web3AuthCacheTrait>) -> Self {
        Self { web3_service }
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

        Ok(ChallengeOutput { nonce, message })
    }
}

#[cfg(test)]
mod tests;
