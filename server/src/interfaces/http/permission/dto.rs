use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::permission::action::Action;

#[derive(Serialize)]
pub struct PermissionEntry {
    pub action: String,
    pub description: String,
    pub category: String,
}

#[derive(Serialize)]
pub struct RolePermissions {
    pub role: String,
    pub permissions: Vec<PermissionEntry>,
}

#[derive(Serialize)]
pub struct GroupPermissionsResponse {
    pub group_id: Uuid,
    pub roles: Vec<RolePermissions>,
}

#[derive(Deserialize)]
pub struct AddPermissionRequest {
    pub action: String,
    pub role: Option<String>,
}

impl From<&Action> for PermissionEntry {
    fn from(action: &Action) -> Self {
        Self {
            action: action.name().to_string(),
            description: action.description().to_string(),
            category: action.category().name().to_string(),
        }
    }
}
