use std::sync::Arc;
use uuid::Uuid;

use crate::errors::app_error::AppError;
use crate::handlers::proposal::{NewMemberRequest, RespondToNewMemberRequest};
use crate::models::group::Group;
use crate::models::proposal::{MyProposalStatus, NewProposal, Proposal, ProposalUpdate};
use crate::models::proposals::new_member::{
    NewMemberProposalExpanded, ReceivedNewMemberProposalExpanded,
};
use crate::models::proposals::withdraw::WithdrawProposalExpanded;
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
    /// Returns a Vector of new member proposals bound to a group
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
    ) -> Result<Vec<ReceivedNewMemberProposalExpanded>, AppError> {
        let result = self
            .proposal_repo
            .find_new_member_received_by(destination)
            .map_err(AppError::Db)?;

        Ok(result)
    }

    pub fn create_new_member_proposal(
        &self,
        created_by: Uuid,
        group_id: Uuid,
        payload: NewMemberRequest,
    ) -> Result<NewMemberProposalExpanded, AppError> {
        // Get user
        let user = match (payload.user_email, payload.user_id) {
            (None, Some(user_id)) => self
                .user_repo
                .find_by_id(user_id)?
                .ok_or(AppError::BadRequest("Usuario no encontrado".to_string()))?,
            (Some(email), _) => self
                .user_repo
                .find_by_email(email)?
                .ok_or(AppError::BadRequest("Usuario no encontrado".to_string()))?,
            (None, None) => {
                return Err(AppError::BadRequest(
                    "Se debe enviar user_id o user_email".to_string(),
                ));
            }
        };

        // validate: new_user not in group
        if self.group_repo.is_member(user.id, group_id)? {
            return Err(AppError::BadRequest(
                "El usuario ya pertenece al grupo".to_string(),
            ));
        }

        let result = self
            .proposal_repo
            .find_new_member_proposal_by_destination_and_group_id(user.id, group_id)?;
        if result.is_some() {
            //TODO when we have votes this should be placed as pending
            let proposal_update = ProposalUpdate {
                status: MyProposalStatus::Approved,
            };
            self.proposal_repo.update_proposal_status(
                result.unwrap().new_member_proposal.proposal_id,
                proposal_update,
            )?;
            return self
                .proposal_repo
                .find_new_member_proposal_by_destination_and_group_id(user.id, group_id)?
                .ok_or(AppError::Internal);
        }

        let result = self.proposal_repo.create_new_member_proposal(
            NewProposal {
                group_id,
                created_by,
            },
            user.id,
        )?;
        /*
        // TODO: Handle voting and only add user to group if proposal is accepted
        // === REMOVE LATER ===
        self.group_repo.add_user_to_group(user.id, group_id)?;
        // === REMOVE LATER ===
        */
        Ok(result)
    }

    pub fn respond_new_member_proposal(
        &self,
        destination: Uuid,
        new_member_proposal_id: Uuid,
        payload: RespondToNewMemberRequest,
    ) -> Result<NewMemberProposalExpanded, AppError> {
        let approve = payload.response;
        let search_proposal = self
            .proposal_repo
            .find_new_member_proposal_by_proposal_id(new_member_proposal_id)?;
        if search_proposal.new_member_proposal.new_member_id != destination {
            return Err(AppError::Forbidden(
                "No podes aceptar una invitacion que no es tuya".into(),
            ));
        }
        if search_proposal.proposal.status != MyProposalStatus::Approved {
            return Err(AppError::BadRequest(
                "Esta invitación ya fue aceptada o rechazada".into(),
            ));
        }
        let next_status = if approve {
            MyProposalStatus::Executed
        } else {
            MyProposalStatus::Rejected
        };

        match self.proposal_repo.respond_to_new_member_proposal(
            new_member_proposal_id,
            destination,
            next_status,
        ) {
            Ok(proposal) => Ok(proposal),
            Err(_) => Err(AppError::BadRequest("Solicitud inválida".parse().unwrap())),
        }
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
                "La propuesta no pertenece al grupo".to_string(),
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
            .ok_or(AppError::BadRequest("La propuesta no existe".to_string()))
    }

    fn find_group(&self, group_id: Uuid) -> Result<Group, AppError> {
        self.group_repo
            .find_by_id(group_id)?
            .ok_or(AppError::BadRequest("El grupo no existe".to_string()))
    }

    pub fn get_all_withdraw_proposals(
        &self,
        group_id: Uuid,
    ) -> Result<Vec<WithdrawProposalExpanded>, AppError> {
        self.proposal_repo
            .get_all_withdraw_proposals(group_id)
            .map(|proposals| proposals.unwrap_or_default())
            .map_err(AppError::Db)
    }
}
