use crate::errors::app_error::AppError;
use crate::models::proposal::Proposal;
use crate::models::user::User;
use crate::repositories::traits::proposal_repo::ProposalRepository;
use crate::repositories::traits::user_repo::UserRepository;
use std::sync::Arc;
use uuid::Uuid;
//use crate::repositories::traits::group_repo::GroupRepository;
use crate::schema::proposal;

#[derive(Clone)]
pub struct ProposalService {
    proposal_repo: Arc<dyn ProposalRepository>,
    //    group_repo: Arc<dyn GroupRepository>
    user_repo: Arc<dyn UserRepository>,
}

impl ProposalService {
    pub fn new(repo: ProposalService) -> Self {
        Self {
            proposal_repo: repo.proposal_repo,
            user_repo: repo.user_repo,
        }
    }

    pub fn get_proposals_group(
        &self,
        group_id: Option<proposal::group_id>,
    ) -> Result<Vec<Proposal>, AppError> {
        let id = group_id.ok_or(AppError::BadRequest("Group ID is required".into()))?;

        // TODO: check if Group exists
        todo!();
        let group_id = Uuid::new_v4(); //get_group(id)?.id;

        let result = self
            .proposal_repo
            .find_by_group(group_id)
            .map_err(AppError::Db)?
            .unwrap_or_default(); // Si es null, devuelve un vector vacíov

        Ok(result)
    }

    /// Get proposals of user
    /// # Errors
    ///
    /// This function can return the following errors:
    ///
    /// - [`AppError::BadRequest`]:
    ///   Returned if `created_by` is `None`. A valid user ID is required.
    ///
    /// - [`AppError::NotFound`]:
    ///   Returned if no user exists with the provided `created_by` ID.
    ///
    /// - [`AppError::Db`]:
    ///   Returned if a database error occurs
    pub fn get_my_proposals(&self, created_by: Option<Uuid>) -> Result<Vec<Proposal>, AppError> {
        let id = created_by.ok_or(AppError::BadRequest("User ID is required".into()))?;

        // Validate user
        let user_id = self.get_user(id)?.id;

        let result = self
            .proposal_repo
            .find_my_proposals(user_id)
            .map_err(AppError::Db)?
            .unwrap_or_default(); // Si es null, devuelve un vector vacío

        Ok(result)
    }

    fn get_user(&self, user_id: Uuid) -> Result<User, AppError> {
        let found_user = self.user_repo.find_by_id(user_id)?;
        found_user.ok_or(AppError::NotFound)
    }

    // fn get_group(&self, group_id: Uuid) -> Result<proposal::group_id, AppError> {
    //     let found_group = self.proposal_repo.find_my_proposals(group_id)?;
    //     found_group.ok_or(AppError::NotFound)
    // }
}
