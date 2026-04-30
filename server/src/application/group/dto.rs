use chrono::{NaiveDate, NaiveDateTime};
use uuid::Uuid;

use crate::infrastructure::db::models::group::{
    GroupMemberStatusModel, GroupRoleModel, GroupStatusModel,
};

pub struct GroupDetails {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub status: GroupStatusModel,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
}

pub struct GroupMemberDetails {
    pub user_id: Uuid,
    pub group_id: Uuid,
    pub name: String,
    pub email: String,
    pub status: GroupMemberStatusModel,
    pub role: GroupRoleModel,
}

pub struct UserInGroupDetails {
    pub user_id: Uuid,
    pub group_id: Uuid,
    pub role: GroupRoleModel,
    pub status: GroupMemberStatusModel,
    pub joined_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub struct GroupFromUserDetails {
    pub user_id: Uuid,
    pub group_id: Uuid,
    pub role: GroupRoleModel,
    pub group_name: String,
    pub group_description: String,
    pub status: GroupStatusModel,
}
