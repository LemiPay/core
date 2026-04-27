use axum::{Json, extract::State};

use crate::application::auth::login::dto::LoginInput;
use crate::interfaces::http::{
    auth::dto::{LoginRequest, LoginResponse},
    error::AppError,
};

use crate::setup::state::AppState;

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    let input = LoginInput {
        email: req.email,
        password: req.password,
    };

    let result = state
        .login_use_case
        .execute(input)
        .map_err(AppError::from)?;

    Ok(Json(LoginResponse {
        token: result.token.0.to_string(),
        user_id: result.user_id.0.to_string(),
    }))
}
