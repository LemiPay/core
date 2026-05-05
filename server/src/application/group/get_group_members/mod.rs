use std::sync::Arc;

use crate::{
    application::group::{dto::GroupMemberDetails, traits::repository::GroupRepository},
    domain::{group::GroupId, user::UserId},
};

#[derive(Debug)]
pub enum GetGroupMembersError {
    Forbidden,
    Internal,
}

#[derive(Clone)]
pub struct GetGroupMembersUseCase {
    pub group_repo: Arc<dyn GroupRepository>,
}

impl GetGroupMembersUseCase {
    pub fn execute(
        &self,
        actor_id: UserId,
        group_id: GroupId,
    ) -> Result<Vec<GroupMemberDetails>, GetGroupMembersError> {
        if !self
            .group_repo
            .is_member(actor_id, group_id)
            .map_err(|_| GetGroupMembersError::Internal)?
        {
            return Err(GetGroupMembersError::Forbidden);
        }

        self.group_repo
            .get_group_members(group_id)
            .map_err(|_| GetGroupMembersError::Internal)
    }
}
