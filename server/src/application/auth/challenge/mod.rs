pub mod dto;

use crate::application::auth::challenge::dto::{ChallengeInput, ChallengeOutput};
use crate::application::auth::traits::web3_auth::Web3AuthTrait;
use crate::interfaces::http::error::AppError;
use chrono::Local;
use moka::sync::Cache;
use std::sync::Arc;
use std::time::Duration;

pub struct ChallengeUseCase {
    pub web3_service: Arc<dyn Web3AuthTrait>,
    nonce_cache: Cache<String, String>,
}

impl ChallengeUseCase {
    pub fn new(web3_service: Arc<dyn Web3AuthTrait>) -> Self {
        Self {
            web3_service,
            nonce_cache: Cache::builder()
                .max_capacity(10_000)
                .time_to_live(Duration::from_secs(900))
                .build(),
        }
    }

    pub fn generate_challenge(&self, input: ChallengeInput) -> Result<ChallengeOutput, AppError> {
        let nonce = self.web3_service.generate_nonce();

        let date_str = Local::now().format("%Y-%m-%d %H:%M").to_string();

        let message = format!(
            "Bienvenido a LemiPay.\n\n\
            Al firmar este mensaje, confirmas que eres el dueño de esta cuenta.\n\n\
            Email: {}\n\
            Nonce: {}\n\
            Fecha: {}",
            input.email, nonce, date_str
        );

        self.nonce_cache.insert(input.email, nonce.clone());

        Ok(ChallengeOutput { nonce, message })
    }
}
