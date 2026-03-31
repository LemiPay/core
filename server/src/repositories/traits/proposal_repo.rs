use crate::data::error::DbError;
use crate::models::proposal::Proposal;
use crate::models::proposals::new_member::NewMemberProposalExpanded;
use uuid::Uuid;

pub trait ProposalRepository: Send + Sync {
    fn find_by_group(&self, group_id: Uuid) -> Result<Vec<NewMemberProposalExpanded>, DbError>;
    fn find_my_proposals(
        &self,
        created_by: Uuid,
    ) -> Result<Vec<NewMemberProposalExpanded>, DbError>;
}
