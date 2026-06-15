use crate::domain::group::GroupId;
use crate::domain::group::member::GroupRole;
use crate::domain::permission::action::Action;

#[derive(Debug, Clone)]
pub struct GroupPermission {
    pub id: String,
    pub group_id: GroupId,
    pub role: GroupRole,
    pub action: Action,
}
