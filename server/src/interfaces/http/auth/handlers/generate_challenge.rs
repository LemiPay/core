use axum::{Json, extract::State};
use crate::application::auth::challenge::dto::{ChallengeInput, ChallengeOutput};
use crate::interfaces::http::auth::dto::{ChallengeRequest, ChallengeResponse};
use crate::interfaces::http::error::AppError;
use crate::setup::state::SharedState;


pub async fn generate_challenge(
    State(state): State<SharedState>,
    Json(req): Json<ChallengeRequest>,
) -> Result<Json<ChallengeResponse>, AppError> {
    let input = ChallengeInput{
        email:req.email,
        address: req.address,
    };
    let res  =  state.auth_service.challenge.generate_challenge(input)?;
    Ok(Json(ChallengeResponse { 
        nonce: res.nonce,
        message: res.message,
    }))
}