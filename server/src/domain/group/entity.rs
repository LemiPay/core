use uuid::Uuid;

use crate::domain::group::config::GroupConfig;
use crate::domain::group::error::GroupError;
use crate::domain::group::member::GroupMember;
use crate::domain::group::policy::GroupPolicy;
use crate::domain::group::types::GroupId;
use crate::domain::user::UserId;

#[derive(Debug, Clone)]
pub struct Group {
    pub id: GroupId,
    pub name: String,
    pub description: String,
    pub is_active: bool,
    pub config: GroupConfig,
    pub members: Vec<GroupMember>,
}

impl Group {
    pub fn new(
        name: String,
        description: String,
        creator_id: UserId,
        config: GroupConfig,
    ) -> Result<Self, GroupError> {
        let trimmed_name = name.trim();
        if trimmed_name.is_empty() {
            return Err(GroupError::InvalidName);
        }
        let trimmed_description = description.trim();
        if trimmed_description.is_empty() {
            return Err(GroupError::InvalidDescription);
        }

        Ok(Self {
            id: GroupId(Uuid::new_v4()),
            name: trimmed_name.to_string(),
            description: trimmed_description.to_string(),
            is_active: true,
            config,
            members: vec![GroupMember::admin(creator_id)],
        })
    }

    pub fn rehydrate(
        id: GroupId,
        name: String,
        description: String,
        is_active: bool,
        config: GroupConfig,
        members: Vec<GroupMember>,
    ) -> Self {
        Self {
            id,
            name,
            description,
            is_active,
            config,
            members,
        }
    }

    pub fn has_member(&self, user_id: UserId) -> bool {
        self.members.iter().any(|m| m.user_id == user_id)
    }

    pub fn member(&self, user_id: UserId) -> Option<&GroupMember> {
        self.members.iter().find(|m| m.user_id == user_id)
    }

    pub fn add_member(
        self,
        actor: &GroupMember,
        new_member: GroupMember,
    ) -> Result<Self, GroupError> {
        GroupPolicy::can_add_member(actor)?;

        if self.has_member(new_member.user_id) {
            return Err(GroupError::UserAlreadyMember);
        }

        let mut members = self.members;
        members.push(new_member);

        Ok(Self { members, ..self })
    }

    pub fn leave_group(self, user_id: UserId) -> Result<Self, GroupError> {
        GroupPolicy::can_leave_group(&self, user_id)?;
        self.remove_member(user_id)
    }

    fn remove_member(self, user_id: UserId) -> Result<Self, GroupError> {
        if !self.has_member(user_id) {
            return Err(GroupError::NotMember);
        }

        let members = self
            .members
            .into_iter()
            .filter(|m| m.user_id != user_id)
            .collect();

        Ok(Self { members, ..self })
    }

    pub fn promote_member_to_admin(mut self, user_id: UserId) -> Result<Self, GroupError> {
        let member = self
            .members
            .iter_mut()
            .find(|m| m.user_id == user_id)
            .ok_or(GroupError::NotMember)?;
        member.role = crate::domain::group::member::GroupRole::Admin;
        Ok(self)
    }

    pub fn deactivate(mut self) -> Self {
        self.is_active = false;
        self
    }

    pub fn update_info(
        mut self,
        name: Option<String>,
        description: Option<String>,
    ) -> Result<Self, GroupError> {
        if name.is_none() && description.is_none() {
            return Err(GroupError::NoFieldsToUpdate);
        }

        if let Some(new_name) = name {
            let trimmed = new_name.trim();
            if trimmed.is_empty() {
                return Err(GroupError::InvalidName);
            }
            self.name = trimmed.to_string();
        }

        if let Some(new_description) = description {
            let trimmed = new_description.trim();
            if trimmed.is_empty() {
                return Err(GroupError::InvalidDescription);
            }
            self.description = trimmed.to_string();
        }

        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    fn user_id() -> UserId {
        UserId(Uuid::new_v4())
    }

    #[test]
    fn new_group_assigns_creator_as_admin() {
        let creator = user_id();
        let group = Group::new(
            "Roomies".into(),
            "Description".into(),
            creator,
            GroupConfig::default(),
        )
        .unwrap();

        assert_eq!(group.members.len(), 1);
        assert_eq!(group.members[0].user_id, creator);
        assert!(group.members[0].is_admin());
        assert!(group.is_active);
    }

    #[test]
    fn new_group_rejects_empty_name() {
        let creator = user_id();
        let result = Group::new("   ".into(), "Description".into(), creator, GroupConfig::default());
        assert!(matches!(result, Err(GroupError::InvalidName)));
    }

    #[test]
    fn admin_can_add_new_member() {
        let admin_id = user_id();
        let new_member_id = user_id();

        let group = Group::new(
            "Roomies".into(),
            "Description".into(),
            admin_id,
            GroupConfig::default(),
        )
        .unwrap();
        let actor = group.member(admin_id).cloned().unwrap();

        let updated = group
            .add_member(&actor, GroupMember::member(new_member_id))
            .unwrap();

        assert_eq!(updated.members.len(), 2);
        assert!(updated.has_member(new_member_id));
    }

    #[test]
    fn non_admin_cannot_add_member() {
        let admin_id = user_id();
        let member_id = user_id();
        let new_member_id = user_id();

        let group = Group::new(
            "Roomies".into(),
            "Description".into(),
            admin_id,
            GroupConfig::default(),
        )
        .unwrap();
        let admin = group.member(admin_id).cloned().unwrap();
        let group = group
            .add_member(&admin, GroupMember::member(member_id))
            .unwrap();

        let plain_member = group.member(member_id).cloned().unwrap();

        let result = group.add_member(&plain_member, GroupMember::member(new_member_id));
        assert!(matches!(result, Err(GroupError::NotAdmin)));
    }

    #[test]
    fn cannot_add_same_member_twice() {
        let admin_id = user_id();
        let new_member_id = user_id();

        let group = Group::new(
            "Roomies".into(),
            "Description".into(),
            admin_id,
            GroupConfig::default(),
        )
        .unwrap();
        let actor = group.member(admin_id).cloned().unwrap();
        let group = group
            .add_member(&actor, GroupMember::member(new_member_id))
            .unwrap();

        let result = group.add_member(&actor, GroupMember::member(new_member_id));
        assert!(matches!(result, Err(GroupError::UserAlreadyMember)));
    }

    #[test]
    fn member_can_leave_group() {
        let admin_id = user_id();
        let member_id = user_id();

        let group = Group::new(
            "Roomies".into(),
            "Description".into(),
            admin_id,
            GroupConfig::default(),
        )
        .unwrap();
        let admin = group.member(admin_id).cloned().unwrap();
        let group = group
            .add_member(&admin, GroupMember::member(member_id))
            .unwrap();

        let group = group.leave_group(member_id).unwrap();
        assert!(!group.has_member(member_id));
        assert!(group.has_member(admin_id));
    }

    #[test]
    fn non_member_cannot_leave_group() {
        let admin_id = user_id();
        let stranger_id = user_id();

        let group = Group::new(
            "Roomies".into(),
            "Description".into(),
            admin_id,
            GroupConfig::default(),
        )
        .unwrap();
        let result = group.leave_group(stranger_id);
        assert!(matches!(result, Err(GroupError::NotMember)));
    }
}
