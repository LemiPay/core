use std::sync::Arc;

use crate::application::group::list_user_groups::dto::{ListUserGroupsInput, ListUserGroupsOutput};
use crate::application::group::list_user_groups::error::ListUserGroupsError;
use crate::application::group::traits::repository::GroupRepository;

#[derive(Clone)]
pub struct ListUserGroupsUseCase {
    pub group_repo: Arc<dyn GroupRepository>,
}

impl ListUserGroupsUseCase {
    pub fn execute(
        &self,
        input: ListUserGroupsInput,
    ) -> Result<ListUserGroupsOutput, ListUserGroupsError> {
        let groups = self
            .group_repo
            .get_user_groups_legacy(input.user_id)
            .map_err(|_| ListUserGroupsError::InternalError)?;

        Ok(ListUserGroupsOutput { groups })
    }
}
