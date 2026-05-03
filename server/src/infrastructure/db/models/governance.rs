use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use diesel_derive_enum::DbEnum;
use serde::Serialize;
use uuid::Uuid;

use crate::domain::governance::{ProposalKind, ProposalStatus};
use crate::infrastructure::db::schema;

#[derive(Debug, DbEnum, Clone, Copy, PartialEq, Eq, Serialize)]
#[db_enum(existing_type_path = "crate::infrastructure::db::schema::sql_types::ProposalStatus")]
pub enum ProposalStatusModel {
    Pending,
    Approved,
    Rejected,
    Executed,
    Canceled,
    Expired,
    Failed,
}

#[derive(Debug, DbEnum, Clone, Copy, PartialEq, Eq, Serialize)]
#[db_enum(existing_type_path = "crate::infrastructure::db::schema::sql_types::VoteType")]
pub enum VoteTypeModel {
    Yes,
    No,
    Abstain,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum ProposalType {
    NewMember,
    FundRound,
    Withdraw,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = schema::proposal)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProposalModel {
    pub id: Uuid,
    pub group_id: Uuid,
    pub created_by: Uuid,
    pub status: ProposalStatusModel,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = schema::proposal)]
pub struct NewProposalModel {
    pub group_id: Uuid,
    pub created_by: Uuid,
}

#[derive(AsChangeset)]
#[diesel(table_name = schema::proposal)]
pub struct ProposalStatusUpdateModel {
    pub status: ProposalStatusModel,
}

#[derive(Queryable, Selectable, Debug, Insertable)]
#[diesel(table_name = schema::new_member_proposal)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewMemberProposalModel {
    pub proposal_id: Uuid,
    pub new_member_id: Uuid,
}

#[derive(Queryable, Selectable, Debug, Insertable)]
#[diesel(table_name = schema::withdraw_proposal)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct WithdrawProposalModel {
    pub proposal_id: Uuid,
    pub amount: BigDecimal,
    pub currency_id: Uuid,
}

#[derive(Queryable, Selectable, Debug, Insertable)]
#[diesel(table_name = schema::fund_round_proposal)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct FundRoundProposalModel {
    pub proposal_id: Uuid,
    pub target_amount: BigDecimal,
    pub currency_id: Uuid,
}

#[derive(Queryable, Selectable, Debug, Insertable)]
#[diesel(table_name = schema::vote)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct VoteModel {
    pub proposal_id: Uuid,
    pub user_id: Uuid,
    pub value: VoteTypeModel,
    pub created_at: NaiveDateTime,
}

#[derive(Queryable, Selectable, Debug, Insertable)]
#[diesel(table_name = schema::fund_round_contribution)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct FundRoundContributionModel {
    pub fund_round_proposal_id: Uuid,
    pub user_id: Uuid,
    pub amount: BigDecimal,
    pub transaction_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = schema::fund_round_contribution)]
pub struct NewFundRoundContributionModel {
    pub fund_round_proposal_id: Uuid,
    pub user_id: Uuid,
    pub amount: BigDecimal,
    pub transaction_id: Uuid,
}

// ----------------------------
// Mappings DB <-> domain
// ----------------------------

impl From<ProposalStatusModel> for ProposalStatus {
    fn from(value: ProposalStatusModel) -> Self {
        match value {
            ProposalStatusModel::Pending => ProposalStatus::Pending,
            ProposalStatusModel::Approved => ProposalStatus::Approved,
            ProposalStatusModel::Rejected => ProposalStatus::Rejected,
            ProposalStatusModel::Executed => ProposalStatus::Executed,
            ProposalStatusModel::Canceled => ProposalStatus::Canceled,
            ProposalStatusModel::Expired => ProposalStatus::Expired,
            ProposalStatusModel::Failed => ProposalStatus::Failed,
        }
    }
}

impl From<ProposalStatus> for ProposalStatusModel {
    fn from(value: ProposalStatus) -> Self {
        match value {
            ProposalStatus::Pending => ProposalStatusModel::Pending,
            ProposalStatus::Approved => ProposalStatusModel::Approved,
            ProposalStatus::Rejected => ProposalStatusModel::Rejected,
            ProposalStatus::Executed => ProposalStatusModel::Executed,
            ProposalStatus::Canceled => ProposalStatusModel::Canceled,
            ProposalStatus::Expired => ProposalStatusModel::Expired,
            ProposalStatus::Failed => ProposalStatusModel::Failed,
        }
    }
}

impl From<ProposalType> for ProposalKind {
    fn from(value: ProposalType) -> Self {
        match value {
            ProposalType::NewMember => ProposalKind::NewMember,
            ProposalType::Withdraw => ProposalKind::Withdraw,
            ProposalType::FundRound => ProposalKind::FundRound,
        }
    }
}

impl From<ProposalKind> for ProposalType {
    fn from(value: ProposalKind) -> Self {
        match value {
            ProposalKind::NewMember => ProposalType::NewMember,
            ProposalKind::Withdraw => ProposalType::Withdraw,
            ProposalKind::FundRound => ProposalType::FundRound,
        }
    }
}
