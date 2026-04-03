use crate::data::state::SharedState;
use crate::errors::app_error::AppError;
use crate::models::user::User;
use crate::security::auth_extractor::AuthUser;
use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: Option<String>,
    pub password: Option<String>,
    pub name: Option<String>,
}

// Register
pub async fn register(
    State(state): State<SharedState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<User>, AppError> {
    let user = state.auth_service.register_user(payload)?;
    Ok(Json(user))
}

/// Login

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: Option<String>,
    pub password: Option<String>,
}

#[derive(Serialize)]
pub struct LoginResponse {
    token: String,
}

pub async fn login(
    State(state): State<SharedState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    let jwt = state.auth_service.login_user(payload)?;
    Ok(Json(LoginResponse { token: jwt }))
}

/// Get my Uuid
#[derive(Serialize)]
pub struct MeResponse {
    id: Uuid,
}

pub async fn get_me(user: AuthUser) -> Result<Json<MeResponse>, AppError> {
    // El parámetro es un User, pero que ya fue validado que esté authed
    Ok(Json(MeResponse { id: user.user_id }))
}
