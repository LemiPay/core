use std::sync::Arc;
use uuid::Uuid;

use crate::errors::app_error::AppError;
use crate::helpers::validations::require_non_empty_uuid;
use crate::models::group::Group;
use crate::models::proposal::{MyProposalStatus, NewProposal, Proposal, ProposalUpdate};
use crate::models::proposals::new_member::NewMemberProposalExpanded;
use crate::models::user::User;
// Repos
use crate::repositories::traits::group_repo::GroupRepository;
use crate::repositories::traits::proposal_repo::ProposalRepository;
use crate::repositories::traits::user_repo::UserRepository;

#[derive(Clone)]
pub struct ProposalService {
    proposal_repo: Arc<dyn ProposalRepository>,
    user_repo: Arc<dyn UserRepository>,
    group_repo: Arc<dyn GroupRepository>,
}

impl ProposalService {
    pub fn new(
        proposal_repo: Arc<dyn ProposalRepository>,
        user_repo: Arc<dyn UserRepository>,
        group_repo: Arc<dyn GroupRepository>,
    ) -> Self {
        Self {
            proposal_repo,
            user_repo,
            group_repo,
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

    pub fn get_received_proposals(
        &self,
        destination: Uuid,
    ) -> Result<Vec<NewMemberProposalExpanded>, AppError> {
        let result = self
            .proposal_repo
            .find_new_member_received_by(destination)
            .map_err(AppError::Db)?;

        Ok(result)
    }

    /// # Create new member proposal
    /// Creates a new proposal for a user to join a group. The proposal is created with
    /// the status "pending" and can be accepted or rejected by an admin of the group.
    /// ### Errors
    ///
    /// This function can return the following errors:
    /// - [`AppError::Db`]:
    ///   Returned if a database error occurs
    /// - [`AppError::BadRequest`]:
    ///   Returned if user not found or already in group
    pub fn create_new_member_proposal(
        &self,
        created_by: Uuid,
        group_id: Uuid,
        new_user_id: Option<Uuid>,
    ) -> Result<NewMemberProposalExpanded, AppError> {
        let new_user = require_non_empty_uuid(new_user_id, "New User ID")?;

        // validate: new_user in exists
        let user = self.find_user(new_user)?;

        // validate: new_user not in group
        if self.group_repo.is_member(user.id, group_id)? {
            return Err(AppError::BadRequest(
                "User is already a member of the group".to_string(),
            ));
        }

        let result = self.proposal_repo.create_new_member_proposal(
            NewProposal {
                group_id,
                created_by,
            },
            new_user,
        )?;

        Ok(result)
    }

    pub fn update_proposal_status(
        &self,
        proposal_id: Uuid,
        status: MyProposalStatus,
    ) -> Result<Proposal, AppError> {
        self.find_proposal(proposal_id)?;

        self.proposal_repo
            .update_proposal_status(proposal_id, ProposalUpdate { status })
            .map_err(AppError::Db)
    }

    pub fn logic_proposal_delete(
        &self,
        proposal_id: Uuid,
        group_id: Uuid,
    ) -> Result<Proposal, AppError> {
        let proposal = self.find_proposal(proposal_id)?;
        let group = self.find_group(proposal.group_id)?;

        if group.id != group_id {
            return Err(AppError::BadRequest(
                "Proposal does not belong to the group".to_string(),
            ));
        }

        self.proposal_repo
            .update_proposal_status(
                proposal_id,
                ProposalUpdate {
                    status: MyProposalStatus::Canceled,
                },
            )
            .map_err(AppError::Db)
    }

    // Helpers with validations
    fn find_proposal(&self, proposal_id: Uuid) -> Result<Proposal, AppError> {
        self.proposal_repo
            .find(proposal_id)?
            .ok_or(AppError::BadRequest("Proposal does not exist".to_string()))
    }

    fn find_group(&self, group_id: Uuid) -> Result<Group, AppError> {
        self.group_repo
            .find_by_id(group_id)?
            .ok_or(AppError::BadRequest("Group does not exist".to_string()))
    }

    fn find_user(&self, user_id: Uuid) -> Result<User, AppError> {
        self.user_repo
            .find_by_id(user_id)?
            .ok_or(AppError::BadRequest("User does not exist".to_string()))
    }
}
