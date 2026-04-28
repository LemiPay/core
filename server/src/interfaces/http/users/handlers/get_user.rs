use axum::Json;
use axum::extract::{Path, State};
use uuid::Uuid;

use crate::application::users::get_user::dto::GetUserInput;

use crate::interfaces::http::{error::AppError, users::dto::GetUserResponse};

use crate::domain::user::UserId;

use crate::setup::state::AppState;

pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<GetUserResponse>, AppError> {
    let output = state
        .user_use_case
        .execute(GetUserInput {
            user_id: UserId(id),
        })
        .map_err(|_| AppError::Internal)?;

    match output.0 {
        None => Err(AppError::NotFound),
        Some(user) => Ok(Json(GetUserResponse {
            id: user.id.to_string(),
            name: user.name.to_string(),
            email: user.email.to_string(),
        })),
    }
}
