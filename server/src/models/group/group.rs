use chrono::NaiveDate;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::group;

#[derive(Debug, DbEnum, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[db_enum(existing_type_path = "crate::schema::sql_types::GroupStatus")]
pub enum MyGroupStatus {
    Active,
    Ended,
}
#[derive(Queryable, Serialize, Selectable)]
#[diesel(table_name = group)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Group {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub status: MyGroupStatus,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
}
#[derive(Insertable, Deserialize)]
#[diesel(table_name = group)]
pub struct NewGroup {
    pub name: String,
    pub description: String,
}
#[derive(Deserialize, AsChangeset)]
#[diesel(table_name = group)]
pub struct GroupUpdate {
    pub name: Option<String>,
    pub description: Option<String>,
    pub status: Option<MyGroupStatus>,
}
