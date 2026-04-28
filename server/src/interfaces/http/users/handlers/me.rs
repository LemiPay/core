use axum::{Json, extract::State};

use crate::interfaces::http::{auth::extractor::AuthUser, error::AppError, users::dto::MeResponse};

use crate::setup::state::AppState;

pub async fn get_me(
    State(state): State<AppState>,
    user: AuthUser,
) -> Result<Json<MeResponse>, AppError> {
    let user = state
        .get_me_use_case
        .execute(user.user_id)
        .map_err(|_| AppError::Internal)?;

    Ok(Json(MeResponse {
        id: user.user_id.to_string(),
        name: user.name.to_string(),
        email: user.email.to_string(),
    }))
}
