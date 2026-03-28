use crate::models::group::MyGroupStatus;
use crate::schema::user_in_group;
use crate::schema::user_in_group::user_id;
use chrono::{DateTime, NaiveDate, NaiveDateTime, Utc};
use diesel::{Insertable, Queryable, Selectable};
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, DbEnum, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[db_enum(existing_type_path = "crate::schema::sql_types::GroupRole")]
pub enum MyGroupRole {
    Admin,
    Member,
}

#[derive(Debug, DbEnum, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[db_enum(existing_type_path = "crate::schema::sql_types::GroupMemberStatus")]
pub enum MyGroupMemberStatus {
    Active,
    Banned,
    Left,
}

#[derive(Queryable, Serialize, Selectable)]
#[diesel(table_name = user_in_group)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserInGroup {
    pub user_id: Uuid,
    pub group_id: Uuid,
    pub role: MyGroupRole,
    pub status: MyGroupMemberStatus,
    pub joined_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = user_in_group)]
pub struct NewGroup {
    pub user_id: Uuid,
    pub group_id: Uuid,
}
