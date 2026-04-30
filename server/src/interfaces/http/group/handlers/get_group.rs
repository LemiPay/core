use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::application::group::get_group::dto::GetGroupInput;

use crate::domain::group::GroupId;

use crate::interfaces::http::{error::AppError, group::dto::GroupResponse};

use crate::setup::state::SharedState;

pub async fn get_group(
    State(state): State<SharedState>,
    Path(id): Path<Uuid>,
) -> Result<Json<GroupResponse>, AppError> {
    let output = state
        .group_service
        .get_group
        .execute(GetGroupInput {
            group_id: GroupId(id),
        })
        .map_err(AppError::from)?;

    let group = output.0.ok_or(AppError::NotFound)?;

    Ok(Json(GroupResponse {
        id: group.id,
        name: group.name,
        description: group.description,
        status: group.status,
        created_at: group.created_at,
        updated_at: group.updated_at,
    }))
}
