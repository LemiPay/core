use crate::schema::{proposal, vote};
use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, DbEnum, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[db_enum(existing_type_path = "crate::schema::sql_types::ProposalStatus")]
pub enum MyProposalStatus {
    Pending,
    Approved,
    Rejected,
    Executed,
    Canceled,
    Expired,
    Failed,
}

#[derive(Debug, DbEnum, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
#[db_enum(existing_type_path = "crate::schema::sql_types::VoteType")]
pub enum MyVoteType {
    Yes,
    No,
    Abstain,
}

#[derive(Serialize)]
pub enum ProposalType {
    NewMember,
    FundRound,
    Withdraw,
}

#[derive(Queryable, Serialize, Selectable)]
#[diesel(table_name = proposal)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Proposal {
    pub id: Uuid,
    pub group_id: Uuid,
    pub created_by: Uuid,
    pub status: MyProposalStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = proposal)]
pub struct NewProposal {
    pub group_id: Uuid,
    pub created_by: Uuid,
}

#[derive(Queryable, Serialize, Selectable)]
#[diesel(table_name = vote)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[allow(dead_code)] // TODO: Remove after implemented
pub struct Vote {
    pub proposal_id: Uuid,
    pub user_id: Uuid,
    pub value: MyVoteType,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = vote)]
pub struct NewVote {
    pub proposal_id: Uuid,
    pub user_id: Uuid,
    pub value: MyVoteType,
}

#[derive(Deserialize, AsChangeset)]
#[diesel(table_name = proposal)]
pub struct ProposalUpdate {
    pub status: MyProposalStatus,
}
