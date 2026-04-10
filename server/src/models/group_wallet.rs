use bigdecimal::BigDecimal;
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = group_wallet)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct GroupWallet {
    pub address: String,
    pub group_id: Uuid,
    pub balance: BigDecimal,
    pub currency_id: Uuid,
    pub created_at: NaiveDate,
    pub updated_at: NaiveDate,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = group_wallet)]
pub struct NewGroupWallet {
    pub address: String,
    pub group_id: Uuid,
    pub currency_id: Uuid,
}
