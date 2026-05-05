use bigdecimal::{BigDecimal, RoundingMode, Zero};
use std::collections::HashMap;
use std::str::FromStr;

use crate::domain::balances::error::BalancesError;
use crate::domain::user::UserId;

/// Immutable representation of per-user balances inside a group.
///
/// All mutations return a brand new `BalancesMap`, leaving the previous
/// instance untouched.
#[derive(Debug, Clone)]
pub struct BalancesMap {
    balances: HashMap<UserId, BigDecimal>,
    group_balance: BigDecimal,
}

impl BalancesMap {
    pub fn new(balances: HashMap<UserId, BigDecimal>, group_balance: BigDecimal) -> Self {
        Self {
            balances,
            group_balance,
        }
    }

    /// Builds an empty map seeded with the given users at zero balance.
    pub fn empty_for(users: &[UserId]) -> Self {
        let mut balances = HashMap::with_capacity(users.len());
        for user_id in users {
            balances.insert(*user_id, BigDecimal::zero());
        }
        Self::new(balances, BigDecimal::zero())
    }

    pub fn get_user_balance(&self, user_id: &UserId) -> Option<&BigDecimal> {
        self.balances.get(user_id)
    }

    pub fn get_group_balance(&self) -> &BigDecimal {
        &self.group_balance
    }

    pub fn get_all_balances(&self) -> &HashMap<UserId, BigDecimal> {
        &self.balances
    }

    /// Returns a new map with `amount` added to `user_id`'s balance and
    /// to the group balance. Errors if the user is not part of the map.
    pub fn add_balance_to_user(
        &self,
        user_id: UserId,
        amount: BigDecimal,
    ) -> Result<BalancesMap, BalancesError> {
        if !self.balances.contains_key(&user_id) {
            return Err(BalancesError::UserNotFound);
        }

        let mut next = self.balances.clone();
        next.entry(user_id)
            .and_modify(|previous| *previous += amount.clone());

        Ok(Self::new(next, self.group_balance.clone() + amount))
    }

    /// Returns a new map with `total_amount` distributed (subtracted)
    /// evenly among all users. Uses 18 decimals and a remainder-spread
    /// strategy so the sum of subtractions matches `total_amount` exactly.
    pub fn remove_to_all(&self, total_amount: BigDecimal) -> Result<BalancesMap, BalancesError> {
        if total_amount > self.group_balance {
            return Err(BalancesError::InsufficientFunds);
        }

        let n = self.balances.len();
        if n == 0 {
            return Ok(self.clone());
        }

        let scale: i64 = 18;
        // Smallest representable unit at the chosen scale.
        let min_unit = BigDecimal::from_str("0.000000000000000001").expect("valid literal");
        let n_decimal = BigDecimal::from(n as u64);

        let amount_per_user =
            (total_amount.clone() / n_decimal.clone()).with_scale_round(scale, RoundingMode::Down);
        let total_deducted = amount_per_user.clone() * n_decimal;
        let mut remainder = total_amount.clone() - total_deducted;

        let mut next = self.balances.clone();
        for user_balance in next.values_mut() {
            let mut deduction = amount_per_user.clone();
            if remainder > BigDecimal::zero() {
                deduction += min_unit.clone();
                remainder -= min_unit.clone();
            }
            *user_balance -= deduction;
        }

        Ok(Self::new(next, self.group_balance.clone() - total_amount))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    fn user() -> UserId {
        UserId(Uuid::new_v4())
    }

    fn dec(value: &str) -> BigDecimal {
        BigDecimal::from_str(value).unwrap()
    }

    #[test]
    fn empty_map_has_zero_balances() {
        let alice = user();
        let bob = user();
        let map = BalancesMap::empty_for(&[alice, bob]);

        assert_eq!(map.get_group_balance(), &BigDecimal::zero());
        assert_eq!(map.get_user_balance(&alice), Some(&BigDecimal::zero()));
        assert_eq!(map.get_user_balance(&bob), Some(&BigDecimal::zero()));
    }

    #[test]
    fn add_balance_credits_user_and_group() {
        let alice = user();
        let initial = BalancesMap::empty_for(&[alice]);

        let updated = initial.add_balance_to_user(alice, dec("100")).unwrap();

        assert_eq!(updated.get_user_balance(&alice).unwrap(), &dec("100"));
        assert_eq!(updated.get_group_balance(), &dec("100"));
    }

    #[test]
    fn add_balance_does_not_mutate_original() {
        let alice = user();
        let initial = BalancesMap::empty_for(&[alice]);

        let _ = initial.add_balance_to_user(alice, dec("100")).unwrap();

        assert_eq!(
            initial.get_user_balance(&alice).unwrap(),
            &BigDecimal::zero()
        );
        assert_eq!(initial.get_group_balance(), &BigDecimal::zero());
    }

    #[test]
    fn add_balance_unknown_user_errors() {
        let alice = user();
        let ghost = user();
        let map = BalancesMap::empty_for(&[alice]);

        assert!(matches!(
            map.add_balance_to_user(ghost, dec("10")),
            Err(BalancesError::UserNotFound)
        ));
    }

    #[test]
    fn remove_to_all_splits_evenly() {
        let alice = user();
        let bob = user();
        let carol = user();
        let map = BalancesMap::empty_for(&[alice, bob, carol])
            .add_balance_to_user(alice, dec("300"))
            .unwrap()
            .add_balance_to_user(bob, dec("300"))
            .unwrap()
            .add_balance_to_user(carol, dec("300"))
            .unwrap();

        let after = map.remove_to_all(dec("150")).unwrap();

        assert_eq!(after.get_user_balance(&alice).unwrap(), &dec("250"));
        assert_eq!(after.get_user_balance(&bob).unwrap(), &dec("250"));
        assert_eq!(after.get_user_balance(&carol).unwrap(), &dec("250"));
        assert_eq!(after.get_group_balance(), &dec("750"));
    }

    #[test]
    fn remove_to_all_distributes_remainder_so_sum_is_exact() {
        let alice = user();
        let bob = user();
        let carol = user();
        let map = BalancesMap::empty_for(&[alice, bob, carol])
            .add_balance_to_user(alice, dec("100"))
            .unwrap()
            .add_balance_to_user(bob, dec("100"))
            .unwrap()
            .add_balance_to_user(carol, dec("100"))
            .unwrap();

        let after = map.remove_to_all(dec("100")).unwrap();

        let sum: BigDecimal = after.get_all_balances().values().cloned().sum();
        assert_eq!(&sum, after.get_group_balance());
        assert_eq!(after.get_group_balance(), &dec("200"));
    }

    #[test]
    fn remove_to_all_overdraft_errors() {
        let alice = user();
        let map = BalancesMap::empty_for(&[alice]);
        assert!(matches!(
            map.remove_to_all(dec("1")),
            Err(BalancesError::InsufficientFunds)
        ));
    }

    #[test]
    fn remove_to_all_with_no_users_is_noop() {
        let map = BalancesMap::empty_for(&[]);
        let after = map.remove_to_all(dec("0")).unwrap();
        assert_eq!(after.get_group_balance(), &BigDecimal::zero());
        assert!(after.get_all_balances().is_empty());
    }
}
