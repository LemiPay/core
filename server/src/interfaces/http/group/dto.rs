use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::infrastructure::db::models::group::{GroupMemberStatusModel, GroupRoleModel, GroupStatusModel};

// ========= Create Group =========

#[derive(Deserialize)]
pub struct CreateGroupRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Serialize)]
pub struct CreateGroupResponse {
    pub id: Uuid,
}

// ========= Get Group =========

#[derive(Serialize)]
pub struct GroupMemberResponse {
    pub user_id: Uuid,
    pub group_id: Uuid,
    pub name: String,
    pub email: String,
    pub status: GroupMemberStatusModel,
    pub role: GroupRoleModel,
}

#[derive(Serialize)]
pub struct GroupResponse {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub status: GroupStatusModel,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
}

#[derive(Serialize)]
pub struct UserInGroupResponse {
    pub user_id: Uuid,
    pub group_id: Uuid,
    pub role: GroupRoleModel,
    pub status: GroupMemberStatusModel,
    pub joined_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

// ========= Leave Group =========

#[derive(Serialize)]
pub struct ListUserGroupsResponse {
    pub user_id: Uuid,
    pub group_id: Uuid,
    pub role: GroupRoleModel,
    pub group_name: String,
    pub group_description: String,
    pub status: GroupStatusModel,
}

#[derive(Deserialize)]
pub struct MakeAdminRequest {
    pub new_user_id: Uuid,
}

#[derive(Deserialize)]
pub struct UpdateGroupRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}
