use crate::errors::app_error::AppError;
use crate::models::user::User;
use crate::security::auth_extractor::AuthUser;
use crate::{data::state::SharedState, models::user::UserSummary};
use axum::{Json, extract::State};
use serde::{Deserialize, Serialize};

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

pub async fn get_me(
    State(state): State<SharedState>,
    user: AuthUser,
) -> Result<Json<UserSummary>, AppError> {
    // El parámetro es un User, pero que ya fue validado que esté authed
    let found_user = state
        .user_service
        .get_user(user.user_id)?
        .ok_or(AppError::NotFound)?;

    Ok(Json(UserSummary {
        id: found_user.id,
        name: found_user.name,
        email: found_user.email,
    }))
}
