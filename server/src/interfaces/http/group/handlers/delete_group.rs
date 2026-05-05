use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    domain::group::GroupId,
    interfaces::http::{auth::extractor::AuthUser, error::AppError, group::dto::GroupResponse},
    setup::state::SharedState,
};

pub async fn delete_group(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<GroupResponse>, AppError> {
    let group = state
        .group_service
        .delete_group
        .execute(user.user_id, GroupId(id))
        .map_err(AppError::from)?;

    Ok(Json(GroupResponse {
        id: group.id,
        name: group.name,
        description: group.description,
        status: group.status,
        created_at: group.created_at,
        updated_at: group.updated_at,
    }))
}
