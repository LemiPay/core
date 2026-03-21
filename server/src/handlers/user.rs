use axum::{
    Json,
    extract::{Path, State},
};
use serde::Deserialize;
use uuid::Uuid;
use crate::data::state::SharedState;
use crate::errors::app_error::AppError;
use crate::models::user::User;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
}

// GET BY ID
pub async fn get_user(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> Result<Json<User>, AppError> {
    let user = state.user_service.get_user(id)?.ok_or(AppError::NotFound)?;

    Ok(Json(user))
}

// LIST
pub async fn list_users(State(state): State<SharedState>) -> Result<Json<Vec<User>>, AppError> {
    let users = state.user_service.list_users()?;

    Ok(Json(users))
}
