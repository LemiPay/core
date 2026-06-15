use std::sync::Arc;

use crate::application::group::create_group::dto::{CreateGroupInput, CreateGroupOutput};
use crate::application::group::create_group::error::CreateGroupError;
use crate::application::group::traits::repository::GroupRepository;
use crate::application::notifications::repository::NotificationRepository;
use crate::domain::group::{Group, GroupConfig};
use validator::ValidateLength;

#[derive(Clone)]
pub struct CreateGroupUseCase {
    pub group_repo: Arc<dyn GroupRepository>,
    pub notification_repo: Arc<dyn NotificationRepository>,
}

impl CreateGroupUseCase {
    pub fn execute(&self, input: CreateGroupInput) -> Result<CreateGroupOutput, CreateGroupError> {
        let is_valid = ValidateLength::validate_length(input.name.trim(), Some(4), Some(30), None)
            && ValidateLength::validate_length(input.description.trim(), Some(8), Some(30), None);
        if !is_valid {
            return Err(CreateGroupError::InvalidName);
        }

        let group = Group::new(
            input.name,
            input.description,
            input.creator_id,
            GroupConfig::default(),
        )?;

        self.group_repo
            .save(&group)
            .map_err(|_| CreateGroupError::InternalError)?;

        // Seed explicit default notification prefs for the creator in this new group
        let _ = self
            .notification_repo
            .initialize_defaults_for_user_in_group(input.creator_id, group.id);

        Ok(CreateGroupOutput { group_id: group.id })
    }
}
