use crate::application::common::repo_error::RepoError;
use crate::application::group::dto::{
    GroupDetails, GroupFromUserDetails, GroupMemberDetails, UserInGroupDetails,
};
use crate::domain::group::{Group, GroupId};
use crate::domain::user::UserId;

pub trait GroupRepository: Send + Sync {
    fn find_by_id(&self, id: GroupId) -> Result<Option<Group>, RepoError>;
    fn save(&self, group: &Group) -> Result<(), RepoError>;
    fn find_by_user(&self, user_id: UserId) -> Result<Vec<Group>, RepoError>;
    fn get_group_details(&self, id: GroupId) -> Result<Option<GroupDetails>, RepoError>;
    fn get_group_members(&self, group_id: GroupId) -> Result<Vec<GroupMemberDetails>, RepoError>;
    fn get_user_groups_legacy(
        &self,
        user_id: UserId,
    ) -> Result<Vec<GroupFromUserDetails>, RepoError>;
    fn get_user_in_group(
        &self,
        user_id: UserId,
        group_id: GroupId,
    ) -> Result<Option<UserInGroupDetails>, RepoError>;
    fn is_member(&self, user_id: UserId, group_id: GroupId) -> Result<bool, RepoError>;
    fn is_admin(&self, user_id: UserId, group_id: GroupId) -> Result<bool, RepoError>;
}
