use crate::data::state::SharedState;
use crate::errors::app_error::AppError;
use crate::models::group::Group;
use crate::models::user_in_group::{GroupFromUser, GroupMember, UserInGroup};
use crate::schema::vote::user_id;
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
    let group_id = state.group_service.create_group(payload, user.user_id);
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
pub async fn get_group_members(
    State(state): State<SharedState>,
    Path(group_id): Path<Uuid>,
) -> Result<Json<Vec<GroupMember>>, AppError> {
    let result = state.group_service.get_group_members(group_id)?;
    Ok(Json(result))
}
pub async fn get_user_groups(
    State(state): State<SharedState>,
    user: AuthUser,
) -> Result<Json<Vec<GroupFromUser>>, AppError> {
    let result = state.group_service.get_user_groups(user.user_id)?;
    Ok(Json(result))
}
