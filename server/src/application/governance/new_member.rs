use uuid::Uuid;

use crate::application::governance::error::GovernanceError;
use crate::domain::governance::{
    GovernancePolicy, NewMemberProposal, Proposal, ProposalId, ProposalKind, ProposalStatus,
};
use crate::domain::group::GroupId;
use crate::domain::user::{Email, UserId};

use super::{
    dto::{NewMemberProposalDetails, ReceivedNewMemberProposalDetails},
    service::GovernanceService,
};

impl GovernanceService {
    pub fn list_group_new_member_proposals(
        &self,
        group_id: Uuid,
    ) -> Result<Vec<NewMemberProposalDetails>, GovernanceError> {
        Self::map_repo(
            self.governance_repo
                .find_group_new_member_proposals(group_id),
        )
    }

    pub fn list_my_new_member_proposals(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<NewMemberProposalDetails>, GovernanceError> {
        Self::map_repo(self.governance_repo.find_my_new_member_proposals(user_id))
    }

    pub fn list_received_new_member_proposals(
        &self,
        user_id: Uuid,
    ) -> Result<Vec<ReceivedNewMemberProposalDetails>, GovernanceError> {
        Self::map_repo(
            self.governance_repo
                .find_received_new_member_proposals(user_id),
        )
    }

    pub fn create_new_member_proposal(
        &self,
        created_by: Uuid,
        group_id: Uuid,
        user_id: Option<Uuid>,
        user_email: Option<String>,
    ) -> Result<NewMemberProposalDetails, GovernanceError> {
        let target_user_id = self.resolve_target_user(user_id, user_email)?;

        if Self::map_repo(
            self.group_repo
                .is_member(UserId(target_user_id), GroupId(group_id)),
        )? {
            return Err(GovernanceError::UserAlreadyMember);
        }

        let existing = Self::map_repo(
            self.governance_repo
                .find_new_member_proposal_by_destination_and_group(target_user_id, group_id),
        )?;

        if let Some(existing) = existing {
            Self::map_repo(
                self.governance_repo
                    .update_proposal_status(existing.proposal.id, Self::initial_proposal_status()),
            )?;
            let refreshed = Self::map_repo(
                self.governance_repo
                    .find_new_member_proposal_by_id(existing.proposal.id),
            )?
            .ok_or(GovernanceError::Internal)?;
            return Ok(refreshed);
        }

        Self::map_repo(self.governance_repo.create_new_member_proposal(
            created_by,
            group_id,
            target_user_id,
        ))
    }

    pub fn respond_new_member_proposal(
        &self,
        destination: Uuid,
        proposal_id: Uuid,
        approve: bool,
    ) -> Result<NewMemberProposalDetails, GovernanceError> {
        let stored = Self::map_repo(
            self.governance_repo
                .find_new_member_proposal_by_id(proposal_id),
        )?
        .ok_or(GovernanceError::NotFound)?;

        let domain = to_domain_new_member(&stored);
        GovernancePolicy::ensure_destination_matches(&domain, UserId(destination))?;
        if domain.proposal.status != ProposalStatus::Approved {
            return Err(GovernanceError::ProposalAlreadyResolved);
        }

        let next = GovernancePolicy::next_status_for_response(approve);
        domain.proposal.status.ensure_can_transition_to(next)?;

        Self::map_repo(self.governance_repo.respond_new_member_proposal(
            proposal_id,
            destination,
            next,
        ))
    }

    fn resolve_target_user(
        &self,
        user_id: Option<Uuid>,
        user_email: Option<String>,
    ) -> Result<Uuid, GovernanceError> {
        match (user_id, user_email) {
            (Some(id), _) => {
                Self::map_repo(self.user_repo.find_by_id(&UserId(id)))?
                    .ok_or(GovernanceError::UserNotFound)?;
                Ok(id)
            }
            (None, Some(email)) => {
                let email = Email::new(email).map_err(|_| GovernanceError::InvalidEmail)?;
                let user = Self::map_repo(self.user_repo.find_by_email(&email))?
                    .ok_or(GovernanceError::UserNotFound)?;
                Ok(user.id)
            }
            (None, None) => Err(GovernanceError::MissingProposalTarget),
        }
    }
}

fn to_domain_new_member(stored: &NewMemberProposalDetails) -> NewMemberProposal {
    NewMemberProposal {
        proposal: Proposal::rehydrate(
            ProposalId(stored.proposal.id),
            GroupId(stored.proposal.group_id),
            UserId(stored.proposal.created_by),
            stored.proposal.status,
            ProposalKind::NewMember,
            stored.proposal.created_at,
            stored.proposal.updated_at,
        ),
        new_member_id: UserId(stored.new_member_id),
    }
}
