use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;

use crate::domain::governance::status::ProposalStatus;
use crate::domain::governance::types::{ProposalId, ProposalKind};
use crate::domain::group::GroupId;
use crate::domain::treasury::CurrencyId;
use crate::domain::user::UserId;

#[derive(Debug, Clone)]
pub struct Proposal {
    pub id: ProposalId,
    pub group_id: GroupId,
    pub created_by: UserId,
    pub status: ProposalStatus,
    pub kind: ProposalKind,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Proposal {
    pub fn rehydrate(
        id: ProposalId,
        group_id: GroupId,
        created_by: UserId,
        status: ProposalStatus,
        kind: ProposalKind,
        created_at: NaiveDateTime,
        updated_at: NaiveDateTime,
    ) -> Self {
        Self {
            id,
            group_id,
            created_by,
            status,
            kind,
            created_at,
            updated_at,
        }
    }
}

#[derive(Debug, Clone)]
pub struct NewMemberProposal {
    pub proposal: Proposal,
    pub new_member_id: UserId,
}

#[derive(Debug, Clone)]
pub struct WithdrawProposal {
    pub proposal: Proposal,
    pub amount: BigDecimal,
    pub currency_id: CurrencyId,
}

#[derive(Debug, Clone)]
pub struct FundRoundProposal {
    pub proposal: Proposal,
    pub target_amount: BigDecimal,
    pub currency_id: CurrencyId,
}
