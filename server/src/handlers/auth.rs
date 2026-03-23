use crate::data::state::SharedState;
use crate::errors::app_error::AppError;
use crate::models::user::User;
use axum::{Json, extract::State};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: Option<String>,
    pub password: Option<String>,
}

// Register
pub async fn register(
    State(state): State<SharedState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<User>, AppError> {
    let user = state.auth_service.register_user(payload)?;
    Ok(Json(user))
}

// Login
pub async fn login(
    State(state): State<SharedState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<User>, AppError> {
    let user = state.auth_service.login_user(payload)?;
    Ok(Json(user))
}
