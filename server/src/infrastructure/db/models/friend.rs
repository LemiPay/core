use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use serde::Serialize;
use uuid::Uuid;

use crate::infrastructure::db::schema;

#[derive(Queryable, Selectable, Serialize, Debug)]
#[diesel(table_name = schema::friend)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct FriendModel {
    pub requester_id: Uuid,
    pub addressee_id: Uuid,
    pub status: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = schema::friend)]
pub struct NewFriendModel {
    pub requester_id: Uuid,
    pub addressee_id: Uuid,
    pub status: String,
}

#[derive(AsChangeset)]
#[diesel(table_name = schema::friend)]
pub struct FriendUpdateModel {
    pub status: String,
    pub updated_at: NaiveDateTime,
}
