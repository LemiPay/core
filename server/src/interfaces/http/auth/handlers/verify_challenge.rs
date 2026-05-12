use crate::interfaces::http::auth::dto::{VerificationRequest, VerificationResponse};
use crate::interfaces::http::error::AppError;
use crate::setup::state::SharedState;
use axum::{Json, extract::State};

pub async fn verify_challenge(
    State(state): State<SharedState>,
    Json(verification_request): Json<VerificationRequest>,
) -> Result<Json<VerificationResponse>, AppError> {
    Ok(Json(VerificationResponse {
        token: "".to_string(),
        user_id: "".to_string(),
    }))
}
