use std::str::FromStr;

use bigdecimal::{BigDecimal, Zero};

use crate::domain::balances::BalancesMap;
use crate::domain::group::GroupStatus;
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

        let epsilon = BigDecimal::from_str("0.000000000001").expect("valid epsilon literal"); // 10^-12
        if balance.abs() >= epsilon {
            return Err(GroupError::BalanceNotZero);
        }
        Ok(())
    }
    pub fn can_end_group(balances: BalancesMap) -> Result<(), GroupError> {
        let epsilon = BigDecimal::from_str("0.000000000001").expect("valid epsilon literal"); // 10^-12
        let non_zero: Vec<String> = balances
            .get_all_balances()
            .iter()
            .filter(|(_, b)| b.abs() >= epsilon)
            .map(|(u, b)| format!("user={} balance={}", u, b))
            .collect();

        if !non_zero.is_empty() {
            eprintln!("[can_end_group] FAILED — balances not zero: {:?}", non_zero);
            return Err(GroupError::NotAllBalancesZero);
        }
        Ok(())
    }
    pub fn can_enter_debt_resolution(user_id: UserId, group: &Group) -> Result<(), GroupError> {
        let member = group.member(user_id).ok_or(GroupError::NotMember)?;
        if !matches!(member.role, GroupRole::Admin) {
            return Err(GroupError::NotAdmin);
        }
        if !matches!(group.status, GroupStatus::Active) {
            return Err(GroupError::GroupNotActive);
        }
        Ok(())
    }
    pub fn ensure_active(group: &Group) -> Result<(), GroupError> {
        if !matches!(group.status, GroupStatus::Active) {
            return Err(GroupError::GroupNotActive);
        }
        Ok(())
    }
    pub fn ensure_in_debt_resolution(group: &Group) -> Result<(), GroupError> {
        if !matches!(group.status, GroupStatus::DebtResolution) {
            return Err(GroupError::GroupNotInDebtResolution);
        }
        Ok(())
    }
}
