use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::domain::governance::{ProposalKind, ProposalStatus};
use crate::infrastructure::db::models::governance::VoteTypeModel;

pub struct ProposalDetails {
    pub id: Uuid,
    pub group_id: Uuid,
    pub created_by: Uuid,
    pub status: ProposalStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub struct NewMemberProposalDetails {
    pub proposal: ProposalDetails,
    pub new_member_id: Uuid,
    pub kind: ProposalKind,
}

pub struct ReceivedNewMemberProposalDetails {
    pub sender_name: String,
    pub group_name: String,
    pub proposal: ProposalDetails,
    pub new_member_id: Uuid,
    pub kind: ProposalKind,
}

pub struct WithdrawProposalDetails {
    pub proposal: ProposalDetails,
    pub amount: BigDecimal,
    pub currency_id: Uuid,
    pub kind: ProposalKind,
}

pub struct FundRoundProposalDetails {
    pub proposal: ProposalDetails,
    pub target_amount: BigDecimal,
    pub currency_id: Uuid,
    pub kind: ProposalKind,
}

pub struct VoteDetails {
    pub proposal_id: Uuid,
    pub user_id: Uuid,
    pub value: VoteTypeModel,
    pub created_at: NaiveDateTime,
}

pub struct FundRoundContributionDetails {
    pub fund_round_proposal_id: Uuid,
    pub user_id: Uuid,
    pub amount: BigDecimal,
    pub transaction_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
