use uuid::Uuid;

use crate::application::governance::error::GovernanceError;
use crate::domain::governance::{
    GovernancePolicy, Proposal, ProposalId, ProposalKind, ProposalStatus,
};
use crate::domain::group::GroupId;
use crate::domain::user::UserId;

use super::{dto::ProposalDetails, service::GovernanceService};

impl GovernanceService {
    pub fn cancel_proposal(
        &self,
        proposal_id: Uuid,
        group_id: Uuid,
    ) -> Result<ProposalDetails, GovernanceError> {
        let stored = Self::map_repo(self.governance_repo.find_proposal(proposal_id))?
            .ok_or(GovernanceError::NotFound)?;

        let proposal = to_domain_proposal(&stored);
        GovernancePolicy::ensure_proposal_in_group(&proposal, GroupId(group_id))?;
        proposal
            .status
            .ensure_can_transition_to(ProposalStatus::Canceled)?;

        Self::map_repo(
            self.governance_repo
                .update_proposal_status(proposal_id, ProposalStatus::Canceled),
        )
    }

    pub fn find_proposal(&self, proposal_id: Uuid) -> Result<ProposalDetails, GovernanceError> {
        Self::map_repo(self.governance_repo.find_proposal(proposal_id))?
            .ok_or(GovernanceError::NotFound)
    }
}

fn to_domain_proposal(stored: &ProposalDetails) -> Proposal {
    Proposal::rehydrate(
        ProposalId(stored.id),
        GroupId(stored.group_id),
        UserId(stored.created_by),
        stored.status,
        ProposalKind::NewMember, // kind not loaded by generic find_proposal; defaults are fine
        stored.created_at,
        stored.updated_at,
    )
}
