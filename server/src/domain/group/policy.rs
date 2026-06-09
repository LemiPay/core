use crate::domain::balances::BalancesMap;
use crate::domain::group::entity::Group;
use crate::domain::group::error::GroupError;
use crate::domain::group::member::{GroupMember, GroupRole};
use crate::domain::user::UserId;
use bigdecimal::Zero;

pub struct GroupPolicy;

impl GroupPolicy {
    pub fn can_add_member(actor: &GroupMember) -> Result<(), GroupError> {
        match actor.role {
            GroupRole::Admin => Ok(()),
            _ => Err(GroupError::NotAdmin),
        }
    }

    pub fn can_leave_group(
        group: &Group,
        user_id: UserId,
        balances: &BalancesMap,
    ) -> Result<(), GroupError> {
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
        let balance = balances
            .get_user_balance(&user_id)
            .ok_or(GroupError::NotMember)?;

        if !balance.is_zero() {
            return Err(GroupError::BalanceNotZero);
        }
        Ok(())
    }
    pub fn can_end_group() -> Result<(), GroupError> {
        todo!("solo se puede hacer si no hay deudas en el grupo")
    }
}
