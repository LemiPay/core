use crate::application::auth::verify_challenge::dto::VerificationInput;
use crate::interfaces::http::auth::dto::{VerificationRequest, VerificationResponse};
use crate::interfaces::http::error::AppError;
use crate::setup::state::SharedState;
use axum::{Json, extract::State};

pub async fn verify_challenge(
    State(state): State<SharedState>,
    Json(verification_request): Json<VerificationRequest>,
) -> Result<Json<VerificationResponse>, AppError> {
    let input = VerificationInput {
        address: verification_request.address.clone(),
        nonce: verification_request.nonce.clone(),
        signature: verification_request.signature.clone(),
        email: verification_request.email.clone(),
        name: verification_request.name.clone(),
        allow_linking: verification_request.allow_linking.unwrap_or(false),
    };

    let res = state
        .auth_service
        .verify_challenge
        .verify_challenge(input)
        .await?;

    Ok(Json(VerificationResponse {
        token: res.token,
        user_id: res.user_id,
    }))
}
