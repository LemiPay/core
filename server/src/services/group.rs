use crate::helpers::validations::is_admin;
use crate::models::group::Group;
use crate::repositories::traits::group_repo::GroupRepository;
use std::sync::Arc;
use uuid::Uuid;

use crate::errors::app_error::AppError;
use crate::handlers::group::NewGroupRequest;

use crate::data::error::DbError;
use crate::data::pool::DbConn;
use crate::helpers::validations::require_non_empty;
use crate::models::user::User;
use crate::models::user_in_group::{GroupFromUser, GroupMember, UserInGroup};
use validator::ValidateLength;

#[derive(Clone)]
pub struct GroupService {
    group_repo: Arc<dyn GroupRepository>,
}
impl GroupService {
    pub fn new(group_repo: Arc<dyn GroupRepository>) -> Self {
        Self { group_repo }
    }
    pub fn get_group_repo(&self) -> Arc<dyn GroupRepository> {
        self.group_repo.clone()
    }
    pub fn create_group(
        &self,
        group: NewGroupRequest,
        id: Uuid,
        conn: DbConn,
    ) -> Result<Uuid, AppError> {
        let name = require_non_empty(group.name, "Name")?;
        let description = require_non_empty(group.description, "Description")?;

        let valid = ValidateLength::validate_length(&name, Some(4), Some(30), None)
            && ValidateLength::validate_length(&description, Some(8), Some(30), None);
        if !valid {
            return Err(AppError::BadRequest("Invalid registration data".into()));
        }

        let group = self.group_repo.create_group(name, description, id);
        let group_id = group?.id;

        Ok(group_id)
    }
    pub fn get_group_by_id(&self, group_id: Uuid) -> Result<Group, AppError> {
        let found_group = self
            .group_repo
            .find_by_id(group_id)?
            .ok_or(AppError::NotFound)?;
        Ok(found_group)
    }
    pub fn make_admin(&self, user_id: Uuid, group_id: Uuid) -> Result<UserInGroup, AppError> {
        if is_admin(user_id, group_id, self.group_repo.clone())? {
            return Err(AppError::Forbidden);
        }
        let result = self.group_repo.make_admin(user_id, group_id)?;
        Ok(result)
    }

    pub fn delete(&self, user_id: Uuid, group_id: Uuid) -> Result<Group, AppError> {
        if is_admin(user_id, group_id, self.group_repo.clone())? {
            return Err(AppError::Forbidden);
        }
        let result = self.group_repo.delete_group(group_id)?;
        Ok(result)
    }

    pub fn get_group_members(&self, group_id: Uuid) -> Result<Vec<GroupMember>, AppError> {
        let result = self.group_repo.get_group_members(group_id)?;
        Ok(result)
    }

    pub fn get_user_groups(&self, user_id: Uuid) -> Result<Vec<GroupFromUser>, AppError> {
        let result = self.group_repo.get_user_groups(user_id)?;

        Ok(result)
    }

    //todo update_group_info fn required -> alta paja ahora
}
