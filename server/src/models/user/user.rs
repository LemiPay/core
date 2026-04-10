use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::user;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub name: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = user)]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub name: String,
}
#[derive(Serialize, Deserialize)]
pub struct UserSummary {
    pub id: Uuid,
    pub email: String,
    pub name: String,
}
