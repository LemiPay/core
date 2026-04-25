use axum::{Json, extract::State};

use crate::application::auth::register::dto::RegisterInput;
use crate::interfaces::http::{
    auth::dto::{RegisterRequest, RegisterResponse},
    error::AppError,
};

use crate::setup::state::AppState;

// Register
pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, AppError> {
    let payload = RegisterInput {
        name: req.name,
        email: req.email,
        password: req.password,
    };

    let result = state
        .register_use_case
        .execute(payload)
        .map_err(AppError::from)?;

    Ok(Json(RegisterResponse {
        user_id: result.user_id.0.to_string(),
    }))
}
