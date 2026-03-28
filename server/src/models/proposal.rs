use crate::schema::proposal;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, Selectable};
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, DbEnum, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[db_enum(existing_type_path = "crate::schema::sql_types::ProposalStatus")]
pub enum ProposalStatus {
    Pending,
    Approved,
    Rejected,
    Executed,
}

#[derive(Queryable, Serialize, Selectable)]
#[diesel(table_name = proposal)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Proposal {
    pub id: Uuid,
    pub group_id: Uuid,
    pub created_by: Uuid,
    pub status: ProposalStatus,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = proposal)]
pub struct NewProposal {
    pub group_id: Uuid,
    pub created_by: Uuid,
}
