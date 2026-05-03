use crate::domain::group::entity::Group;
use crate::domain::group::error::GroupError;
use crate::domain::group::member::{GroupMember, GroupRole};
use crate::domain::user::UserId;

pub struct GroupPolicy;

impl GroupPolicy {
    pub fn can_add_member(actor: &GroupMember) -> Result<(), GroupError> {
        match actor.role {
            GroupRole::Admin => Ok(()),
            _ => Err(GroupError::NotAdmin),
        }
    }

    pub fn can_leave_group(group: &Group, user_id: UserId) -> Result<(), GroupError> {
        let member = group.member(user_id).ok_or(GroupError::NotMember)?;
        if matches!(member.role, GroupRole::Admin) {
            let has_other_admin = group
                .members
                .iter()
                .any(|m| m.user_id != user_id && matches!(m.role, GroupRole::Admin));
            if !has_other_admin {
                return Err(GroupError::LastAdminCannotLeave);
            }
        }
        Ok(())
    }
}
