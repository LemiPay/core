use crate::models::proposal::{Proposal, ProposalType};
use crate::schema::withdraw_proposal;
use bigdecimal::BigDecimal;
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Insertable, Serialize, Deserialize, Selectable)]
#[diesel(table_name = withdraw_proposal)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct WithdrawProposal {
    pub proposal_id: Uuid,
    pub amount: BigDecimal,
}

#[derive(Serialize)]
#[allow(dead_code)] // TODO: remove after implemented
pub struct WithdrawProposalExpanded {
    pub proposal: Proposal,
    pub withdraw_proposal: WithdrawProposal,
    pub proposal_type: ProposalType,
}

#[derive(Serialize)]
#[allow(dead_code)] // TODO: remove after implemented
pub struct ReceivedWithdrawProposalExpanded {
    pub sender_name: String,
    pub group_name: String,
    pub proposal: Proposal,
    pub withdraw_proposal: WithdrawProposal,
    pub proposal_type: ProposalType,
}
