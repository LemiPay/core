use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::currency;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = currency)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Currency {
    pub currency_id: Uuid,
    pub name: String,
    pub ticker: String,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = currency)]
pub struct NewCurrency {
    pub name: String,
    pub ticker: String,
}
