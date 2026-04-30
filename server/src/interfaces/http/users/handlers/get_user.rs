use axum::Json;
use axum::extract::{Path, State};
use uuid::Uuid;

use crate::application::users::{error::UserError, get_user::dto::GetUserInput};

use crate::interfaces::http::{error::AppError, users::dto::GetUserResponse};

use crate::domain::user::UserId;

use crate::setup::state::SharedState;

pub async fn get_user(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> Result<Json<GetUserResponse>, AppError> {
    let output = state
        .user_service
        .get_user
        .execute(GetUserInput {
            user_id: UserId(id),
        })
        .map_err(|err| match err {
            UserError::NotFound => AppError::NotFound,
            _ => AppError::Internal,
        })?
        .0
        .ok_or(AppError::NotFound)?;

    Ok(Json(GetUserResponse {
        id: output.id.to_string(),
        name: output.name.to_string(),
        email: output.email.to_string(),
    }))
}
