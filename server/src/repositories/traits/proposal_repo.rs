use bigdecimal::BigDecimal;
use uuid::Uuid;

use crate::data::error::DbError;
use crate::models::proposal::{MyProposalStatus, NewProposal, Proposal, ProposalUpdate};
use crate::models::proposals::new_member::{
    NewMemberProposalExpanded, ReceivedNewMemberProposalExpanded,
};
use crate::models::proposals::withdraw::WithdrawProposalExpanded;

pub trait ProposalRepository: Send + Sync {
    fn find_by_group(&self, group_id: Uuid) -> Result<Vec<NewMemberProposalExpanded>, DbError>;
    fn find_my_proposals(
        &self,
        created_by: Uuid,
    ) -> Result<Vec<NewMemberProposalExpanded>, DbError>;

    fn respond_to_new_member_proposal(
        &self,
        new_member_proposal_id: Uuid,
        user_id: Uuid,
        next_status: MyProposalStatus,
    ) -> Result<NewMemberProposalExpanded, DbError>;

    fn find_new_member_received_by(
        &self,
        destination: Uuid,
    ) -> Result<Vec<ReceivedNewMemberProposalExpanded>, DbError>;
    fn find(&self, proposal_id: Uuid) -> Result<Option<Proposal>, DbError>;

    fn find_new_member_proposal_by_destination_and_group_id(
        &self,
        destination: Uuid,
        group_id: Uuid,
    ) -> Result<Option<NewMemberProposalExpanded>, DbError>;
    fn find_new_member_proposal_by_proposal_id(
        &self,
        proposal_id: Uuid,
    ) -> Result<NewMemberProposalExpanded, DbError>;

    fn create_new_member_proposal(
        &self,
        new_proposal: NewProposal,
        new_user_id: Uuid,
    ) -> Result<NewMemberProposalExpanded, DbError>;

    fn update_proposal_status(
        &self,
        proposal_id: Uuid,
        params: ProposalUpdate,
    ) -> Result<Proposal, DbError>;

    fn create_withdraw_proposal(
        &self,
        user_id: Uuid,
        group_id: Uuid,
        currency_id: Uuid,
        amount: BigDecimal,
    ) -> Result<WithdrawProposalExpanded, DbError>;

    fn find_withdraw_proposal(
        &self,
        proposal_id: Uuid,
        currency_id: Uuid,
    ) -> Result<Option<WithdrawProposalExpanded>, DbError>;

    fn get_all_withdraw_proposals(
        &self,
        group_id: Uuid,
    ) -> Result<Option<Vec<WithdrawProposalExpanded>>, DbError>;
}
