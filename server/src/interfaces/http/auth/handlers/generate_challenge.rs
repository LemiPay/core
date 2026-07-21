use crate::application::auth::challenge::dto::ChallengeInput;
use crate::interfaces::http::auth::dto::{ChallengeRequest, ChallengeResponse};
use crate::interfaces::http::error::AppError;
use crate::setup::state::SharedState;
use axum::{Json, extract::State};

pub async fn generate_challenge(
    State(state): State<SharedState>,
    Json(req): Json<ChallengeRequest>,
) -> Result<Json<ChallengeResponse>, AppError> {
    let input = ChallengeInput {
        address: req.address,
    };

    let res = state.auth_service.challenge.generate_challenge(input)?;

    Ok(Json(ChallengeResponse {
        nonce: res.nonce,
        message: res.message,
        is_linked: res.is_linked,
        issued_at: res.issued_at,
    }))
}
