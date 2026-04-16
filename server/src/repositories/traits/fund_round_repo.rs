use bigdecimal::BigDecimal;
use uuid::Uuid;

use crate::data::error::DbError;
use crate::models::proposal::{NewProposal, Proposal, ProposalUpdate};
use crate::models::proposals::fund_round::FundProposalExpanded;
use crate::models::transaction::fund_round_contrib::FundRoundContribution;

pub trait FundRoundRepository: Send + Sync {
    fn create_fund_round_proposal(
        &self,
        new_proposal: NewProposal,
        target_amount: BigDecimal,
        currency_id: Uuid,
    ) -> Result<FundProposalExpanded, DbError>;

    fn find_fund_round(&self, fund_round_id: Uuid)
    -> Result<Option<FundProposalExpanded>, DbError>;

    fn get_total_contributed(&self, fund_round_id: Uuid) -> Result<BigDecimal, DbError>;

    fn create_contribution(
        &self,
        fund_round_id: Uuid,
        user_id: Uuid,
        amount: BigDecimal,
        sender_wallet_id: Uuid,
    ) -> Result<FundRoundContribution, DbError>;

    fn update_proposal_status(
        &self,
        proposal_id: Uuid,
        params: ProposalUpdate,
    ) -> Result<Proposal, DbError>;
}
