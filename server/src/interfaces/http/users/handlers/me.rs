use crate::application::users::{error::UserError, get_user::dto::GetUserInput};

use crate::interfaces::http::{
    auth::extractor::AuthUser, error::AppError, users::dto::GetUserResponse,
};

use crate::setup::state::SharedState;
use axum::{Json, extract::State};

pub async fn get_me(
    State(state): State<SharedState>,
    user: AuthUser,
) -> Result<Json<GetUserResponse>, AppError> {
    let output = state
        .user_service
        .get_user
        .execute(GetUserInput {
            user_id: user.user_id,
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
