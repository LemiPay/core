use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::application::group::leave_group::dto::LeaveGroupInput;
use crate::domain::group::GroupId;
use crate::interfaces::http::{
    auth::extractor::AuthUser, error::AppError, group::dto::UserInGroupResponse,
};

use crate::setup::state::SharedState;

pub async fn leave_group(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<Json<UserInGroupResponse>, AppError> {
    let input = LeaveGroupInput {
        group_id: GroupId(id),
        user_id: user.user_id,
    };

    let output = state
        .group_service
        .leave_group
        .execute(input)
        .map_err(AppError::from)?;

    Ok(Json(UserInGroupResponse {
        user_id: output.relation.user_id,
        group_id: output.relation.group_id,
        role: output.relation.role,
        status: output.relation.status,
        joined_at: output.relation.joined_at,
        updated_at: output.relation.updated_at,
    }))
}
