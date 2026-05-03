use std::collections::HashSet;

use bigdecimal::{BigDecimal, Zero};

use crate::domain::expense::entity::Expense;
use crate::domain::expense::error::ExpenseError;
use crate::domain::group::GroupId;
use crate::domain::user::UserId;

/// Pure business rules for expenses.
///
/// All functions are pure and side-effect free; the application layer
/// orchestrates persistence around them.
pub struct ExpensePolicy;

impl ExpensePolicy {
    /// Amount must be strictly positive.
    pub fn ensure_positive_amount(amount: &BigDecimal) -> Result<(), ExpenseError> {
        if *amount <= BigDecimal::zero() {
            Err(ExpenseError::InvalidAmount)
        } else {
            Ok(())
        }
    }

    /// Participant list must be non-empty and have no duplicated `UserId`.
    pub fn ensure_unique_non_empty_participants(
        participants: &[UserId],
    ) -> Result<(), ExpenseError> {
        if participants.is_empty() {
            return Err(ExpenseError::EmptyParticipants);
        }

        let mut seen: HashSet<UserId> = HashSet::with_capacity(participants.len());
        for user_id in participants {
            if !seen.insert(*user_id) {
                return Err(ExpenseError::DuplicatedParticipant);
            }
        }
        Ok(())
    }

    /// The expense must not be in `Deleted` state.
    pub fn ensure_not_deleted(expense: &Expense) -> Result<(), ExpenseError> {
        if expense.status.is_deleted() {
            Err(ExpenseError::AlreadyDeleted)
        } else {
            Ok(())
        }
    }

    /// The caller must be the original creator of the expense.
    pub fn ensure_owner(expense: &Expense, caller: UserId) -> Result<(), ExpenseError> {
        if expense.user_id == caller {
            Ok(())
        } else {
            Err(ExpenseError::NotOwner)
        }
    }

    /// The expense must belong to the provided group.
    pub fn ensure_in_group(expense: &Expense, group_id: GroupId) -> Result<(), ExpenseError> {
        if expense.group_id == group_id {
            Ok(())
        } else {
            Err(ExpenseError::GroupMismatch)
        }
    }

    /// Splits the total amount equally among `n` participants.
    ///
    /// Note: replicates the legacy behaviour `total / n`. Any rounding
    /// remainder is intentionally left as a future improvement and is
    /// encapsulated here so callers don't have to change.
    pub fn split_amount_equally(
        total: &BigDecimal,
        participants_count: usize,
    ) -> Result<BigDecimal, ExpenseError> {
        if participants_count == 0 {
            return Err(ExpenseError::EmptyParticipants);
        }
        let divisor: i64 =
            i64::try_from(participants_count).map_err(|_| ExpenseError::TooManyParticipants)?;
        Ok(total / BigDecimal::from(divisor))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bigdecimal::FromPrimitive;
    use chrono::NaiveDateTime;
    use uuid::Uuid;

    use crate::domain::expense::entity::Expense;
    use crate::domain::expense::status::ExpenseStatus;
    use crate::domain::expense::types::ExpenseId;
    use crate::domain::treasury::CurrencyId;

    fn make_expense(user: UserId, group: GroupId, status: ExpenseStatus) -> Expense {
        let now: NaiveDateTime = NaiveDateTime::default();
        Expense::rehydrate(
            ExpenseId(Uuid::new_v4()),
            user,
            group,
            CurrencyId(Uuid::new_v4()),
            None,
            BigDecimal::from_u32(100).unwrap(),
            status,
            now,
            now,
        )
    }

    #[test]
    fn rejects_zero_amount() {
        let zero = BigDecimal::zero();
        assert_eq!(
            ExpensePolicy::ensure_positive_amount(&zero),
            Err(ExpenseError::InvalidAmount)
        );
    }

    #[test]
    fn rejects_negative_amount() {
        let negative = BigDecimal::from(-1);
        assert_eq!(
            ExpensePolicy::ensure_positive_amount(&negative),
            Err(ExpenseError::InvalidAmount)
        );
    }

    #[test]
    fn accepts_positive_amount() {
        let positive = BigDecimal::from(1);
        assert_eq!(ExpensePolicy::ensure_positive_amount(&positive), Ok(()));
    }

    #[test]
    fn rejects_empty_participants() {
        let participants: Vec<UserId> = vec![];
        assert_eq!(
            ExpensePolicy::ensure_unique_non_empty_participants(&participants),
            Err(ExpenseError::EmptyParticipants)
        );
    }

    #[test]
    fn rejects_duplicated_participants() {
        let user = UserId(Uuid::new_v4());
        let participants = vec![user, user];
        assert_eq!(
            ExpensePolicy::ensure_unique_non_empty_participants(&participants),
            Err(ExpenseError::DuplicatedParticipant)
        );
    }

    #[test]
    fn accepts_unique_non_empty_participants() {
        let participants = vec![UserId(Uuid::new_v4()), UserId(Uuid::new_v4())];
        assert_eq!(
            ExpensePolicy::ensure_unique_non_empty_participants(&participants),
            Ok(())
        );
    }

    #[test]
    fn rejects_split_with_zero_participants() {
        let total = BigDecimal::from(100);
        assert_eq!(
            ExpensePolicy::split_amount_equally(&total, 0),
            Err(ExpenseError::EmptyParticipants)
        );
    }

    #[test]
    fn splits_amount_equally_among_participants() {
        let total = BigDecimal::from(90);
        let per_participant = ExpensePolicy::split_amount_equally(&total, 3).unwrap();
        assert_eq!(per_participant, BigDecimal::from(30));
    }

    #[test]
    fn deleted_expense_rejected() {
        let expense = make_expense(
            UserId(Uuid::new_v4()),
            GroupId(Uuid::new_v4()),
            ExpenseStatus::Deleted,
        );
        assert_eq!(
            ExpensePolicy::ensure_not_deleted(&expense),
            Err(ExpenseError::AlreadyDeleted)
        );
    }

    #[test]
    fn non_deleted_expense_accepted() {
        let expense = make_expense(
            UserId(Uuid::new_v4()),
            GroupId(Uuid::new_v4()),
            ExpenseStatus::Created,
        );
        assert_eq!(ExpensePolicy::ensure_not_deleted(&expense), Ok(()));
    }

    #[test]
    fn ensure_owner_rejects_non_owner() {
        let owner = UserId(Uuid::new_v4());
        let other = UserId(Uuid::new_v4());
        let expense = make_expense(owner, GroupId(Uuid::new_v4()), ExpenseStatus::Created);
        assert_eq!(
            ExpensePolicy::ensure_owner(&expense, other),
            Err(ExpenseError::NotOwner)
        );
        assert_eq!(ExpensePolicy::ensure_owner(&expense, owner), Ok(()));
    }

    #[test]
    fn ensure_in_group_rejects_other_group() {
        let group = GroupId(Uuid::new_v4());
        let other_group = GroupId(Uuid::new_v4());
        let expense = make_expense(UserId(Uuid::new_v4()), group, ExpenseStatus::Created);
        assert_eq!(
            ExpensePolicy::ensure_in_group(&expense, other_group),
            Err(ExpenseError::GroupMismatch)
        );
        assert_eq!(ExpensePolicy::ensure_in_group(&expense, group), Ok(()));
    }
}
