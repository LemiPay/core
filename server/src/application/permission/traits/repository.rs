use crate::application::common::repo_error::RepoError;
use crate::domain::group::GroupId;
use crate::domain::group::member::GroupRole;
use crate::domain::permission::action::Action;

pub trait PermissionRepository: Send + Sync {
    fn is_action_allowed(
        &self,
        group_id: GroupId,
        role: GroupRole,
        action: &Action,
    ) -> Result<bool, RepoError>;

    fn find_by_group(&self, group_id: GroupId) -> Result<Vec<(GroupRole, Action)>, RepoError>;

    fn add_permission(
        &self,
        group_id: GroupId,
        role: GroupRole,
        action: &Action,
    ) -> Result<(), RepoError>;

    fn remove_permission(
        &self,
        group_id: GroupId,
        role: GroupRole,
        action: &Action,
    ) -> Result<(), RepoError>;

    fn seed_defaults(&self, group_id: GroupId) -> Result<(), RepoError>;
}
