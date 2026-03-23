use crate::data::state::SharedState;
use crate::errors::app_error::AppError;
use crate::models::user::User;
use axum::{Json, extract::State};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

// CREATE
pub async fn register(
    State(state): State<SharedState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<User>, AppError> {
    let user = state.auth_service.register_user(payload)?;
    Ok(Json(user))
}
