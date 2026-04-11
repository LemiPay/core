use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::schema::user_wallet;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = user_wallet)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UserWallet {
    pub id: Uuid,
    pub address: String,
    pub user_id: Uuid,
    pub currency_id: Uuid,
    pub balance: BigDecimal,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = user_wallet)]
pub struct NewUserWallet {
    pub address: String,
    pub user_id: Uuid,
    pub currency_id: Uuid,
}
