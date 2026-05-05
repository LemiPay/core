use crate::domain::user::UserId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroupRole {
    Admin,
    Member,
}

#[derive(Debug, Clone)]
pub struct GroupMember {
    pub user_id: UserId,
    pub role: GroupRole,
}

impl GroupMember {
    pub fn new(user_id: UserId, role: GroupRole) -> Self {
        Self { user_id, role }
    }

    pub fn admin(user_id: UserId) -> Self {
        Self::new(user_id, GroupRole::Admin)
    }

    pub fn member(user_id: UserId) -> Self {
        Self::new(user_id, GroupRole::Member)
    }

    pub fn is_admin(&self) -> bool {
        matches!(self.role, GroupRole::Admin)
    }
}
