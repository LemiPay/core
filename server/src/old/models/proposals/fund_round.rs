use crate::models::proposal::{Proposal, ProposalType};
use crate::schema::fund_round_proposal;
use bigdecimal::BigDecimal;
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Insertable, Serialize, Deserialize, Selectable)]
#[diesel(table_name = fund_round_proposal)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct FundProposal {
    pub proposal_id: Uuid,
    pub target_amount: BigDecimal,
    pub currency_id: Uuid,
}

#[derive(Serialize)]
pub struct FundProposalExpanded {
    pub proposal: Proposal,
    pub fund_round_proposal: FundProposal,
    pub proposal_type: ProposalType,
}

#[derive(Serialize)]
#[allow(dead_code)] // TODO: remove after implemented
pub struct ReceivedFundRoundProposalExpanded {
    pub sender_name: String,
    pub group_name: String,
    pub proposal: Proposal,
    pub fund_round_proposal: FundProposal,
    pub proposal_type: ProposalType,
}
