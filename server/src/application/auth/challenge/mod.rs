pub mod dto;

use std::sync::Arc;
use crate::application::auth::challenge::dto::{ChallengeInput, ChallengeOutput};
use crate::application::auth::traits::web3_auth::Web3AuthTrait;
use crate::interfaces::http::error::AppError;
use chrono::Local;

pub struct ChallengeUseCase {
    pub web3_service: Arc<dyn Web3AuthTrait>
}


impl ChallengeUseCase {
    pub fn generate_challenge(&self, input: ChallengeInput) -> Result<ChallengeOutput, AppError> {
        let nonce = self.web3_service.generate_nonce();
        let message = format!(
            "Bienvenido a LemiPay.\n\n\
            Al firmar este mensaje, confirmas que eres el dueño de esta cuenta.\n\n\
            Email: {}\n\
            Nonce: {}\n\
            Fecha: {}",
            input.email,
            nonce,
            Local::now().to_rfc3339()
        );
        Ok(ChallengeOutput{
            nonce: nonce.to_string(),
            message: message.to_string(),
        })
    }
}