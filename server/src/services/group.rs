use std::sync::Arc;
use uuid::Uuid;

// Handlers
use crate::handlers::group::NewGroupRequest;

// Models
use crate::models::group::{Group, GroupUpdate};
use crate::models::user_in_group::{GroupFromUser, GroupMember, MyGroupRole, UserInGroup};

// Repos
use crate::repositories::traits::group_repo::GroupRepository;

// Utils
use crate::errors::app_error::AppError;
use crate::helpers::validations::require_non_empty;
use validator::ValidateLength;

#[derive(Clone)]
pub struct GroupService {
    group_repo: Arc<dyn GroupRepository>,
}
impl GroupService {
    pub fn new(group_repo: Arc<dyn GroupRepository>) -> Self {
        Self { group_repo }
    }

    pub fn create_group(&self, group: NewGroupRequest, id: Uuid) -> Result<Uuid, AppError> {
        let name = require_non_empty(group.name, "Name")?;
        let description = require_non_empty(group.description, "Description")?;

        let valid = ValidateLength::validate_length(&name, Some(4), Some(30), None)
            && ValidateLength::validate_length(&description, Some(8), Some(30), None);

        if !valid {
            return Err(AppError::BadRequest(
                "Invalid group name or description".into(),
            ));
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

    pub fn is_member(&self, user_id: Uuid, group_id: Uuid) -> Result<bool, AppError> {
        let result = self.group_repo.is_member(user_id, group_id)?;
        Ok(result)
    }

    pub fn is_admin(&self, user_id: Uuid, group_id: Uuid) -> Result<bool, AppError> {
        let result = self.group_repo.is_admin(user_id, group_id)?;
        Ok(result)
    }

    pub fn make_admin(&self, user_id: Uuid, group_id: Uuid) -> Result<UserInGroup, AppError> {
        // Validate if user in group
        if !self.group_repo.is_member(user_id, group_id)? {
            return Err(AppError::BadRequest("User not in group".into()));
        }

        // Validate if not admin
        if self.is_admin(user_id, group_id)? {
            return Err(AppError::BadRequest("User is already an admin".into()));
        }

        let result = self.group_repo.make_admin(user_id, group_id)?;
        Ok(result)
    }

    pub fn delete(&self, user_id: Uuid, group_id: Uuid) -> Result<Group, AppError> {
        if !self.is_admin(user_id, group_id)? {
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

    pub fn leave_group(&self, user_id: Uuid, group_id: Uuid) -> Result<UserInGroup, AppError> {
        if self.is_admin(user_id, group_id)? {
            let members = self.get_group_members(group_id)?;

            let has_other_admin = members
                .iter()
                .any(|m| m.user_id != user_id && m.role.eq(&MyGroupRole::Admin));

            if !has_other_admin {
                return Err(AppError::BadRequest(
                    "Group does not have the another admin".into(),
                ));
            }
        }

        self.group_repo
            .remove_user_from_group(user_id, group_id)
            .map_err(AppError::Db)
    }

    pub fn update_group(
        &self,
        user_id: Uuid,
        group_id: Uuid,
        update: GroupUpdate,
    ) -> Result<Group, AppError> {
        if !self.is_admin(user_id, group_id)? {
            return Err(AppError::Forbidden);
        }

        if update.name.is_none() && update.description.is_none() {
            return Err(AppError::BadRequest("No fields to update".into()));
        }

        if let Some(ref name) = update.name {
            if !ValidateLength::validate_length(name.trim(), Some(4), Some(30), None) {
                return Err(AppError::BadRequest(
                    "Invalid group name: must be 4–30 characters".into(),
                ));
            }
        }

        if let Some(ref description) = update.description {
            if !ValidateLength::validate_length(description.trim(), Some(8), Some(30), None) {
                return Err(AppError::BadRequest(
                    "Invalid group description: must be 8–30 characters".into(),
                ));
            }
        }

        let result = self.group_repo.update_group_info(group_id, update)?;
        Ok(result)
    }
}
