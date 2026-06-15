use std::sync::Arc;

use crate::application::common::repo_error::RepoError;
use crate::application::group::traits::repository::GroupRepository;
use crate::application::permission::error::PermissionError;
use crate::application::permission::traits::repository::PermissionRepository;
use crate::domain::group::GroupId;
use crate::domain::permission::action::Action;
use crate::domain::user::UserId;

#[derive(Clone)]
pub struct PermissionService {
    pub permission_repo: Arc<dyn PermissionRepository>,
    pub group_repo: Arc<dyn GroupRepository>,
}

impl PermissionService {
    pub fn check_allowed(
        &self,
        user_id: UserId,
        group_id: GroupId,
        action: &Action,
    ) -> Result<(), PermissionError> {
        let user_in_group = self
            .group_repo
            .get_user_in_group(user_id, group_id)
            .map_err(|_| PermissionError::Internal)?
            .ok_or(PermissionError::NotMember)?;

        let allowed = self
            .permission_repo
            .is_action_allowed(group_id, user_in_group.role.into(), action)
            .map_err(|_| PermissionError::Internal)?;

        if allowed {
            Ok(())
        } else {
            Err(PermissionError::ActionNotAllowed)
        }
    }
}

impl From<RepoError> for PermissionError {
    fn from(_: RepoError) -> Self {
        PermissionError::Internal
    }
}
