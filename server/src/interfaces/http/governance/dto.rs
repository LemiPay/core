use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::governance::dto::{
    FundRoundContributionDetails, FundRoundProposalDetails, NewMemberProposalDetails,
    ProposalDetails, ReceivedNewMemberProposalDetails, WithdrawProposalDetails,
};
use crate::domain::governance::{ProposalKind, ProposalStatus};

#[derive(Deserialize)]
pub struct NewMemberRequest {
    pub user_id: Option<Uuid>,
    pub user_email: Option<String>,
}

#[derive(Deserialize)]
pub struct RespondProposalRequest {
    pub response: bool,
}

#[derive(Deserialize)]
pub struct WithdrawProposalRequest {
    pub address: String,
    pub amount: String,
    pub currency_id: Uuid,
}

#[derive(Deserialize)]
pub struct ExecuteWithdrawRequest {
    pub address: String,
    pub proposal_id: Uuid,
    pub currency_id: Uuid,
}

#[derive(Deserialize)]
pub struct CreateFundRoundRequest {
    pub target_amount: String,
    pub currency_id: Uuid,
}

#[derive(Deserialize)]
pub struct ContributeFundRoundRequest {
    pub amount: String,
    pub sender_wallet_id: Uuid,
}

#[derive(Serialize)]
pub enum ProposalStatusResponse {
    Pending,
    Approved,
    Rejected,
    Executed,
    Canceled,
    Expired,
    Failed,
}

impl From<ProposalStatus> for ProposalStatusResponse {
    fn from(value: ProposalStatus) -> Self {
        match value {
            ProposalStatus::Pending => Self::Pending,
            ProposalStatus::Approved => Self::Approved,
            ProposalStatus::Rejected => Self::Rejected,
            ProposalStatus::Executed => Self::Executed,
            ProposalStatus::Canceled => Self::Canceled,
            ProposalStatus::Expired => Self::Expired,
            ProposalStatus::Failed => Self::Failed,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProposalKindResponse {
    NewMember,
    Withdraw,
    FundRound,
}

impl From<ProposalKind> for ProposalKindResponse {
    fn from(value: ProposalKind) -> Self {
        match value {
            ProposalKind::NewMember => Self::NewMember,
            ProposalKind::Withdraw => Self::Withdraw,
            ProposalKind::FundRound => Self::FundRound,
        }
    }
}

#[derive(Serialize)]
pub struct ProposalResponse {
    pub id: Uuid,
    pub group_id: Uuid,
    pub created_by: Uuid,
    pub status: ProposalStatusResponse,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize)]
pub struct NewMemberProposalResponse {
    pub proposal: ProposalResponse,
    pub new_member_id: Uuid,
    pub kind: ProposalKindResponse,
}

#[derive(Serialize)]
pub struct ReceivedNewMemberProposalResponse {
    pub sender_name: String,
    pub group_name: String,
    pub proposal: ProposalResponse,
    pub new_member_id: Uuid,
    pub kind: ProposalKindResponse,
}

#[derive(Serialize)]
pub struct WithdrawProposalResponse {
    pub proposal: ProposalResponse,
    pub amount: BigDecimal,
    pub currency_id: Uuid,
    pub kind: ProposalKindResponse,
}

#[derive(Serialize)]
pub struct FundRoundProposalResponse {
    pub proposal: ProposalResponse,
    pub target_amount: BigDecimal,
    pub currency_id: Uuid,
    pub kind: ProposalKindResponse,
}

#[derive(Serialize)]
pub struct FundRoundContributionResponse {
    pub fund_round_proposal_id: Uuid,
    pub user_id: Uuid,
    pub amount: BigDecimal,
    pub transaction_id: Uuid,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Serialize)]
pub struct FundRoundStatusResponse {
    pub fund_round: FundRoundProposalResponse,
    pub total_contributed: String,
    pub target_amount: String,
    pub is_completed: bool,
}

#[derive(Serialize)]
pub struct FundRoundRemainingResponse {
    pub remaining: String,
    pub is_last_contributor: bool,
}

impl From<ProposalDetails> for ProposalResponse {
    fn from(value: ProposalDetails) -> Self {
        Self {
            id: value.id,
            group_id: value.group_id,
            created_by: value.created_by,
            status: value.status.into(),
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl From<NewMemberProposalDetails> for NewMemberProposalResponse {
    fn from(value: NewMemberProposalDetails) -> Self {
        Self {
            proposal: value.proposal.into(),
            new_member_id: value.new_member_id,
            kind: value.kind.into(),
        }
    }
}

impl From<ReceivedNewMemberProposalDetails> for ReceivedNewMemberProposalResponse {
    fn from(value: ReceivedNewMemberProposalDetails) -> Self {
        Self {
            sender_name: value.sender_name,
            group_name: value.group_name,
            proposal: value.proposal.into(),
            new_member_id: value.new_member_id,
            kind: value.kind.into(),
        }
    }
}

impl From<WithdrawProposalDetails> for WithdrawProposalResponse {
    fn from(value: WithdrawProposalDetails) -> Self {
        Self {
            proposal: value.proposal.into(),
            amount: value.amount,
            currency_id: value.currency_id,
            kind: value.kind.into(),
        }
    }
}

impl From<FundRoundProposalDetails> for FundRoundProposalResponse {
    fn from(value: FundRoundProposalDetails) -> Self {
        Self {
            proposal: value.proposal.into(),
            target_amount: value.target_amount,
            currency_id: value.currency_id,
            kind: value.kind.into(),
        }
    }
}

impl From<FundRoundContributionDetails> for FundRoundContributionResponse {
    fn from(value: FundRoundContributionDetails) -> Self {
        Self {
            fund_round_proposal_id: value.fund_round_proposal_id,
            user_id: value.user_id,
            amount: value.amount,
            transaction_id: value.transaction_id,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}
