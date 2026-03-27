use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::group;
#[derive(Queryable, Serialize, Selectable)]
#[diesel(table_name = group)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Group {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}
#[derive(Insertable, Deserialize)]
#[diesel(table_name = group)]
pub struct NewGroup {
    pub name: String,
    pub description: String,
}