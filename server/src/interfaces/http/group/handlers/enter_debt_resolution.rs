use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    application::group::enter_debt_resolution::dto::EnterDebtResolutionInput,
    domain::group::GroupId,
    interfaces::http::{auth::extractor::AuthUser, error::AppError, group::dto::GroupResponse},
    setup::state::SharedState,
};

pub async fn enter_debt_resolution(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<GroupResponse>, AppError> {
    let output = state
        .group_service
        .enter_debt_resolution
        .execute(EnterDebtResolutionInput {
            group_id: GroupId(id),
            actor_id: user.user_id,
        })
        .map_err(AppError::from)?;

    Ok(Json(GroupResponse {
        id: output.group.id,
        name: output.group.name,
        description: output.group.description,
        status: output.group.status,
        created_at: output.group.created_at,
        updated_at: output.group.updated_at,
    }))
}
