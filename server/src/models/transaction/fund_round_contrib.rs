use crate::schema::fund_round_contribution;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Selectable, Serialize)]
#[diesel(table_name = fund_round_contribution)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[allow(dead_code)]
pub struct FundRoundContribution {
    pub fund_round_proposal_id: Uuid,
    pub user_id: Uuid,

    pub amount: BigDecimal,
    pub transaction_id: Uuid,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = fund_round_contribution)]
pub struct NewFundRoundContribution {
    pub fund_round_proposal_id: Uuid,
    pub user_id: Uuid,

    pub amount: BigDecimal,
    pub transaction_id: Uuid,
}
