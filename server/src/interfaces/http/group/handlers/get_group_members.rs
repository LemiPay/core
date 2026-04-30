use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    domain::group::GroupId,
    interfaces::http::{
        auth::extractor::AuthUser,
        error::AppError,
        group::dto::GroupMemberResponse,
    },
    setup::state::SharedState,
};

pub async fn get_group_members(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<GroupMemberResponse>>, AppError> {
    let members = state
        .group_service
        .get_group_members
        .execute(user.user_id, GroupId(id))
        .map_err(AppError::from)?;

    Ok(Json(
        members
            .into_iter()
            .map(|m| GroupMemberResponse {
                user_id: m.user_id,
                group_id: m.group_id,
                name: m.name,
                email: m.email,
                status: m.status,
                role: m.role,
            })
            .collect(),
    ))
}
