use crate::models::group::Group;
use crate::repositories::traits::group_repo::GroupRepository;
use crate::repositories::traits::user_in_group_repo::UserInGroupRepo;
use std::sync::Arc;
use uuid::Uuid;

use crate::errors::app_error::AppError;
use crate::handlers::group::NewGroupRequest;

use crate::helpers::validations::require_non_empty;
use validator::ValidateLength;

#[derive(Clone)]
pub struct GroupService {
    group_repo: Arc<dyn GroupRepository>,
    user_in_group_repo: Arc<dyn UserInGroupRepo>,
}
impl GroupService {
    pub fn new(
        group_repo: Arc<dyn GroupRepository>,
        user_in_group_repo: Arc<dyn UserInGroupRepo>,
    ) -> Self {
        Self {
            group_repo,
            user_in_group_repo,
        }
    }
    pub fn create_group(&self, group: NewGroupRequest, id: Uuid) -> Result<Uuid, AppError> {
        let name = require_non_empty(group.name, "Name")?;
        let description = require_non_empty(group.description, "Description")?;

        let valid = ValidateLength::validate_length(&name, Some(4), Some(30), None)
            && ValidateLength::validate_length(&description, Some(8), Some(30), None);
        if !valid {
            return Err(AppError::BadRequest("Invalid registration data".into()));
        }

        //todo manejar estas dos acciones como transacciones de SQL
        let group = self.group_repo.create_group(name, description);
        let group_id = group?.id;

        let user_in_group = self.user_in_group_repo.add_user_to_group(id, group_id);

        Ok(group_id)
    }
    pub fn get_group_by_id(&self, group_id: Uuid) -> Result<Group, AppError> {
        let found_group = self
            .group_repo
            .find_by_id(group_id)?
            .ok_or(AppError::NotFound)?;
        Ok(found_group)
    }
}
