use axum::{
    Json,
    extract::{Path, State},
};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::errors::app_error::AppError;
use crate::models::user::User;
use crate::services::user::UserService;

#[derive(Clone)]
pub struct AppState {
    pub user_service: Arc<UserService>,
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

// CREATE
pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<User>, AppError> {
    let user = state
        .user_service
        .create_user(payload.name, payload.email)?;
    Ok(Json(user))
}

// GET BY ID
pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<User>, AppError> {
    let user = state.user_service.get_user(id)?.ok_or(AppError::NotFound)?;

    Ok(Json(user))
}

// LIST
pub async fn list_users(State(state): State<AppState>) -> Result<Json<Vec<User>>, AppError> {
    let users = state.user_service.list_users()?;

    Ok(Json(users))
}
