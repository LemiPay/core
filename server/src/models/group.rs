use diesel::{AsExpression, FromSqlRow, Insertable, Queryable, Selectable};
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::group;


//esto estuve viendo y no puede usar de una el enum de postgres asi que hay que crear nuestro
//model de enum en rust

#[derive(Debug, Serialize, Deserialize, AsExpression, FromSqlRow)]
#[diesel(sql_type = crate::schema::sql_types::GroupStatus)]
pub enum MyGroupStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "ended")]
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
    pub description: String
}