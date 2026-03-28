use crate::models::group::Group;
use crate::models::user::User;
use crate::repositories::traits::group_repo::GroupRepository;
use std::sync::Arc;
use uuid::Uuid;

use crate::errors::app_error::AppError;
use crate::handlers::group::NewGroupRequest;

use crate::helpers::validations::require_non_empty;
use validator::ValidateLength;

#[derive(Clone)]
pub struct GroupService {
    repo: Arc<dyn GroupRepository>,
}
impl GroupService {
    pub fn new(repo: Arc<dyn GroupRepository>) -> Self {
        Self { repo }
    }
    pub fn create_group(&self, group: NewGroupRequest, id: Uuid) -> Result<Uuid, AppError> {
        let name = require_non_empty(group.name, "Name")?;
        let description = require_non_empty(group.description, "Description")?;

        let valid = ValidateLength::validate_length(&name, Some(4), Some(30), None)
            && ValidateLength::validate_length(&description, Some(8), Some(30), None);
        if !valid {
            return Err(AppError::BadRequest("Invalid registration data".into()));
        }
        let group = self.repo.create_group(name, description);
        //todo crear la tupla de user in group

        Ok(group?.id)
    }
    pub fn get_group_by_id(&self, group_id: Uuid) -> Result<Group, AppError> {
        let found_group = self.repo.find_by_id(group_id)?.ok_or(AppError::NotFound)?;
        Ok(found_group)
    }
}
