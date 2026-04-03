use crate::data::error::DbError;
use crate::models::proposal::{NewProposal, Proposal, ProposalUpdate};
use crate::models::proposals::new_member::NewMemberProposalExpanded;
use uuid::Uuid;

pub trait ProposalRepository: Send + Sync {
    fn find_by_group(&self, group_id: Uuid) -> Result<Vec<NewMemberProposalExpanded>, DbError>;
    fn find_my_proposals(
        &self,
        created_by: Uuid,
    ) -> Result<Vec<NewMemberProposalExpanded>, DbError>;

    fn find_new_member_received_by(
        &self,
        destination: Uuid,
    ) -> Result<Vec<NewMemberProposalExpanded>, DbError>;
    fn find(&self, proposal_id: Uuid) -> Result<Option<Proposal>, DbError>;

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
}
