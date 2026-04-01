use crate::data::state::SharedState;
use crate::errors::app_error::AppError;
use crate::models::group::Group;
use crate::models::user_in_group::UserInGroup;
use crate::security::auth_extractor::AuthUser;
use axum::{
    Json,
    extract::{Path, State},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct NewGroupRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}
#[derive(Serialize)]
pub struct NewGroupResponse {
    id: Uuid,
}
#[derive(Deserialize)]
pub struct NewMakeAdminRequest {
    pub new_user_id: Uuid,
}
pub async fn create_group(
    State(state): State<SharedState>,
    user: AuthUser,
    Json(payload): Json<NewGroupRequest>,
) -> Result<Json<NewGroupResponse>, AppError> {
    let conn = state.db.get_conn()?;
    let group_id = state
        .group_service
        .create_group(payload, user.user_id, conn);
    Ok(Json(NewGroupResponse { id: group_id? }))
}
pub async fn get_group_by_id(
    State(state): State<SharedState>,
    Path(group_id): Path<Uuid>,
) -> Result<Json<Group>, AppError> {
    let group = state.group_service.get_group_by_id(group_id);
    Ok(group?.into())
}

pub async fn make_group_admin(
    State(state): State<SharedState>,
    Path(group_id): Path<Uuid>,
    Json(payload): Json<NewMakeAdminRequest>,
) -> Result<Json<UserInGroup>, AppError> {
    let result = state
        .group_service
        .make_admin(payload.new_user_id, group_id)?;
    Ok(Json(result))
}
