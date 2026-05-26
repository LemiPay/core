use bigdecimal::{BigDecimal, Zero};

use super::{entity::Investment, error::InvestmentError, status::InvestmentStatus};

pub struct InvestmentPolicy;

impl InvestmentPolicy {
    pub fn ensure_positive_amount(amount: &BigDecimal) -> Result<(), InvestmentError> {
        if *amount <= BigDecimal::zero() {
            Err(InvestmentError::InvalidAmount)
        } else {
            Ok(())
        }
    }

    pub fn ensure_can_mature(investment: &Investment) -> Result<(), InvestmentError> {
        investment
            .status
            .ensure_can_transition_to(InvestmentStatus::Matured)
    }

    pub fn ensure_can_withdraw(investment: &Investment) -> Result<(), InvestmentError> {
        if investment.status != InvestmentStatus::Matured {
            return Err(InvestmentError::NotMatured);
        }
        investment
            .status
            .ensure_can_transition_to(InvestmentStatus::Withdrawn)
    }

    pub fn calculate_invested_percentage() {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bigdecimal::FromPrimitive;
    use chrono::Utc;
    use uuid::Uuid;

    use crate::domain::{
        governance::ProposalId,
        group::GroupId,
        investment::{
            entity::Investment,
            status::InvestmentStatus,
            types::{InvestmentId, InvestmentStrategyId},
        },
        treasury::CurrencyId,
        user::UserId,
    };

    fn make_investment(status: InvestmentStatus) -> Investment {
        let now = Utc::now().naive_utc();
        Investment::rehydrate(
            InvestmentId(Uuid::new_v4()),
            GroupId(Uuid::new_v4()),
            ProposalId(Uuid::new_v4()),
            InvestmentStrategyId(Uuid::new_v4()),
            CurrencyId(Uuid::new_v4()),
            BigDecimal::from_u64(1000).unwrap(),
            BigDecimal::from_u64(50).unwrap(),
            None,
            status,
            now,
            now + chrono::TimeDelta::days(30),
            now,
            now,
        )
    }

    #[test]
    fn rejects_zero_amount() {
        let zero = BigDecimal::from_u64(0).unwrap();
        assert!(matches!(
            InvestmentPolicy::ensure_positive_amount(&zero),
            Err(InvestmentError::InvalidAmount)
        ));
    }

    #[test]
    fn can_withdraw_only_when_matured() {
        let active = make_investment(InvestmentStatus::Active);
        assert!(matches!(
            InvestmentPolicy::ensure_can_withdraw(&active),
            Err(InvestmentError::NotMatured)
        ));

        let matured = make_investment(InvestmentStatus::Matured);
        assert!(InvestmentPolicy::ensure_can_withdraw(&matured).is_ok());
    }

    #[test]
    fn can_mature_only_when_active() {
        let active = make_investment(InvestmentStatus::Active);
        assert!(InvestmentPolicy::ensure_can_mature(&active).is_ok());

        let matured = make_investment(InvestmentStatus::Matured);
        assert!(matches!(
            InvestmentPolicy::ensure_can_mature(&matured),
            Err(InvestmentError::InvalidStatusTransition)
        ));
    }
}
