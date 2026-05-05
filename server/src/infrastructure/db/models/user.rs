use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::infrastructure::db::schema;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = schema::user)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserModel {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub name: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = schema::user)]
pub struct NewUserModel {
    pub email: String,
    pub password: String,
    pub name: String,
}
// #[derive(Serialize, Deserialize)]
// pub struct UserSummary {
//     pub id: Uuid,
//     pub email: String,
//     pub name: String,
// }
