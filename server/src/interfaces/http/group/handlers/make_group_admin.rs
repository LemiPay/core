use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;

use crate::{
    domain::{group::GroupId, user::UserId},
    interfaces::http::{
        auth::extractor::AuthUser,
        error::AppError,
        group::dto::{MakeAdminRequest, UserInGroupResponse},
    },
    setup::state::SharedState,
};

pub async fn make_group_admin(
    State(state): State<SharedState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
    Json(req): Json<MakeAdminRequest>,
) -> Result<Json<UserInGroupResponse>, AppError> {
    let relation = state
        .group_service
        .make_group_admin
        .execute(user.user_id, GroupId(id), UserId(req.new_user_id))
        .map_err(AppError::from)?;

    Ok(Json(UserInGroupResponse {
        user_id: relation.user_id,
        group_id: relation.group_id,
        role: relation.role,
        status: relation.status,
        joined_at: relation.joined_at,
        updated_at: relation.updated_at,
    }))
}
