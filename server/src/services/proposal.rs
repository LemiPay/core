use std::sync::Arc;
use uuid::Uuid;

use crate::errors::app_error::AppError;
use crate::helpers::validations::{require_non_empty, require_non_empty_uuid};
use crate::models::proposal::{NewProposal, Proposal};
use crate::models::proposals::new_member::{NewMemberProposal, NewMemberProposalExpanded};
// Repos
use crate::repositories::traits::proposal_repo::ProposalRepository;
use crate::repositories::traits::user_repo::UserRepository;

#[derive(Clone)]
pub struct ProposalService {
    proposal_repo: Arc<dyn ProposalRepository>,
    user_repo: Arc<dyn UserRepository>,
}

impl ProposalService {
    pub fn new(
        proposal_repo: Arc<dyn ProposalRepository>,
        user_repo: Arc<dyn UserRepository>,
    ) -> Self {
        Self {
            proposal_repo,
            user_repo,
        }
    }

    /// # Get proposals of group
    /// Returns a Vector of proposals bound to a group
    ///
    /// ### Errors
    ///
    /// This function can return the following errors:
    /// - [`AppError::Db`]:
    ///   Returned if a database error occurs
    pub fn get_proposals_group(
        &self,
        group_id: Uuid,
    ) -> Result<Vec<NewMemberProposalExpanded>, AppError> {
        // check if Group exists (IN MIDDLEWARE)
        let result = self
            .proposal_repo
            .find_by_group(group_id)
            .map_err(AppError::Db)?;

        Ok(result)
    }

    /// # Get proposals of user
    /// Returns a Vector of proposals created by a user
    ///
    /// ### Errors
    ///
    /// This function can return the following errors:
    /// - [`AppError::Db`]:
    ///   Returned if a database error occurs
    pub fn get_my_proposals(
        &self,
        created_by: Uuid,
    ) -> Result<Vec<NewMemberProposalExpanded>, AppError> {
        let result = self
            .proposal_repo
            .find_my_proposals(created_by)
            .map_err(AppError::Db)?;

        Ok(result)
    }

    /// # Create new member proposal
    /// Creates a new proposal for a user to join a group. The proposal is created with
    /// the status "pending" and can be accepted or rejected by an admin of the group.
    pub fn create_new_member_proposal(
        &self,
        created_by: Uuid,
        group_id: Uuid,
        new_user_id: Option<Uuid>,
    ) -> Result<NewMemberProposalExpanded, AppError> {
        // TODO: validate: new_user in group
        // TODO: validate: new_user not in group
        let new_member_id = require_non_empty_uuid(new_user_id, "New User ID")?;

        let result = self.proposal_repo.create_new_member_proposal(
            NewProposal {
                group_id,
                created_by,
            },
            new_member_id,
        )?;

        Ok(result)
    }
}
