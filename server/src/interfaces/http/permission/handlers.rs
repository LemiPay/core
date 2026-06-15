use axum::{
    Json,
    extract::{Path, Query, State},
};
use uuid::Uuid;

use crate::domain::group::member::GroupRole;
use crate::domain::permission::action::Action;
use crate::interfaces::http::{
    error::AppError,
    permission::dto::{
        AddPermissionRequest, GroupPermissionsResponse, PermissionEntry, RolePermissions,
    },
};
use crate::setup::state::SharedState;

#[derive(serde::Deserialize)]
pub struct RemovePermissionQuery {
    pub action: String,
    pub role: Option<String>,
}

pub async fn list_permissions(
    State(state): State<SharedState>,
    Path(group_id): Path<Uuid>,
) -> Result<Json<GroupPermissionsResponse>, AppError> {
    let permissions = state
        .permission_service
        .list_permissions(crate::domain::group::GroupId(group_id))
        .map_err(AppError::from)?;

    let mut admin_entries = Vec::new();
    let mut member_entries = Vec::new();

    for (role, action) in &permissions {
        let entry = PermissionEntry::from(action);
        match role {
            GroupRole::Admin => admin_entries.push(entry),
            GroupRole::Member => member_entries.push(entry),
        }
    }

    Ok(Json(GroupPermissionsResponse {
        group_id,
        roles: vec![
            RolePermissions {
                role: "Admin".into(),
                permissions: admin_entries,
            },
            RolePermissions {
                role: "Member".into(),
                permissions: member_entries,
            },
        ],
    }))
}

pub async fn add_permission(
    State(state): State<SharedState>,
    Path(group_id): Path<Uuid>,
    Json(payload): Json<AddPermissionRequest>,
) -> Result<Json<()>, AppError> {
    let action = Action::from_name(&payload.action)
        .ok_or_else(|| AppError::BadRequest(format!("Unknown action: {}", payload.action)))?;

    let role = match payload.role.as_deref().unwrap_or("Member") {
        "Admin" => GroupRole::Admin,
        "Member" => GroupRole::Member,
        _ => {
            return Err(AppError::BadRequest(
                "Role must be 'Admin' or 'Member'".into(),
            ));
        }
    };

    state
        .permission_service
        .add_permission(crate::domain::group::GroupId(group_id), role, &action)
        .map_err(AppError::from)?;

    Ok(Json(()))
}

pub async fn remove_permission(
    State(state): State<SharedState>,
    Path(group_id): Path<Uuid>,
    Query(params): Query<RemovePermissionQuery>,
) -> Result<Json<()>, AppError> {
    let action = Action::from_name(&params.action)
        .ok_or_else(|| AppError::BadRequest(format!("Unknown action: {}", params.action)))?;

    let role = match params.role.as_deref().unwrap_or("Member") {
        "Admin" => GroupRole::Admin,
        "Member" => GroupRole::Member,
        _ => {
            return Err(AppError::BadRequest(
                "Role must be 'Admin' or 'Member'".into(),
            ));
        }
    };

    state
        .permission_service
        .remove_permission(crate::domain::group::GroupId(group_id), role, &action)
        .map_err(AppError::from)?;

    Ok(Json(()))
}
