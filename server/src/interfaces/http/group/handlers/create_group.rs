use axum::{Json, extract::State};

use crate::application::group::create_group::dto::CreateGroupInput;
use crate::domain::group::GroupId;

use crate::interfaces::http::{
    auth::extractor::AuthUser,
    error::AppError,
    group::dto::{CreateGroupRequest, CreateGroupResponse},
};

use crate::setup::state::SharedState;

pub async fn create_group(
    State(state): State<SharedState>,
    user: AuthUser,
    Json(req): Json<CreateGroupRequest>,
) -> Result<Json<CreateGroupResponse>, AppError> {
    let name = req
        .name
        .ok_or_else(|| AppError::BadRequest("Nombre requerido".into()))?;
    let description = req
        .description
        .ok_or_else(|| AppError::BadRequest("Descripción requerida".into()))?;

    let input = CreateGroupInput {
        name: name.clone(),
        description,
        creator_id: user.user_id,
    };

    let output = state
        .group_service
        .create_group
        .execute(input)
        .map_err(AppError::from)?;

    // Seed a notification for the creator (and any initial members) so email can fire if prefs allow.
    state
        .notification_service
        .notify_group_event("new_member_added", GroupId(output.group_id.0), &name)
        .await;

    Ok(Json(CreateGroupResponse {
        id: output.group_id.0,
    }))
}
