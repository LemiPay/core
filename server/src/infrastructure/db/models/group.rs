use chrono::{NaiveDate, NaiveDateTime};
use diesel::{Insertable, Queryable, Selectable};
use diesel_derive_enum::DbEnum;
use serde::Serialize;
use uuid::Uuid;

use crate::domain::group::{GroupMember, GroupRole};
use crate::domain::user::UserId;
use crate::infrastructure::db::schema;

// ----------------------------
// Enums
// ----------------------------

#[derive(Debug, DbEnum, Clone, Copy, PartialEq, Eq, Serialize)]
#[db_enum(existing_type_path = "crate::infrastructure::db::schema::sql_types::GroupStatus")]
pub enum GroupStatusModel {
    Active,
    Ended,
}

#[derive(Debug, DbEnum, Clone, Copy, PartialEq, Eq, Serialize)]
#[db_enum(existing_type_path = "crate::infrastructure::db::schema::sql_types::GroupRole")]
pub enum GroupRoleModel {
    Admin,
    Member,
}

#[derive(Debug, DbEnum, Clone, Copy, PartialEq, Eq, Serialize)]
#[db_enum(existing_type_path = "crate::infrastructure::db::schema::sql_types::GroupMemberStatus")]
pub enum GroupMemberStatusModel {
    Active,
    Banned,
    Left,
}

// ----------------------------
// Group rows
// ----------------------------

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = schema::group)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GroupModel {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub status: GroupStatusModel,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
}

#[derive(Insertable)]
#[diesel(table_name = schema::group)]
pub struct NewGroupModel {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

// ----------------------------
// User-in-group rows
// ----------------------------

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = schema::user_in_group)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserInGroupModel {
    pub user_id: Uuid,
    pub group_id: Uuid,
    pub role: GroupRoleModel,
    pub status: GroupMemberStatusModel,
    pub joined_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = schema::user_in_group)]
pub struct NewUserInGroupModel {
    pub user_id: Uuid,
    pub group_id: Uuid,
    pub role: GroupRoleModel,
}

// ----------------------------
// Mappings DB <-> domain
// ----------------------------

impl From<GroupRoleModel> for GroupRole {
    fn from(role: GroupRoleModel) -> Self {
        match role {
            GroupRoleModel::Admin => GroupRole::Admin,
            GroupRoleModel::Member => GroupRole::Member,
        }
    }
}

impl From<GroupRole> for GroupRoleModel {
    fn from(role: GroupRole) -> Self {
        match role {
            GroupRole::Admin => GroupRoleModel::Admin,
            GroupRole::Member => GroupRoleModel::Member,
        }
    }
}

impl From<UserInGroupModel> for GroupMember {
    fn from(row: UserInGroupModel) -> Self {
        GroupMember {
            user_id: UserId(row.user_id),
            role: row.role.into(),
        }
    }
}
