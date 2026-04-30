use axum::{Json, extract::State};

use crate::application::auth::register::dto::RegisterInput;

use crate::interfaces::http::{
    auth::dto::{RegisterRequest, RegisterResponse},
    error::AppError,
};

use crate::setup::state::SharedState;

// Register
pub async fn register(
    State(state): State<SharedState>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, AppError> {
    let payload = RegisterInput {
        name: req.name,
        email: req.email,
        password: req.password,
    };

    let result = state
        .auth_service
        .register
        .execute(payload)
        .map_err(AppError::from)?;

    Ok(Json(RegisterResponse {
        user_id: result.user_id.0.to_string(),
    }))
}
