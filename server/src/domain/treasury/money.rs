use bigdecimal::{BigDecimal, Zero};

use crate::domain::treasury::{currency::CurrencyId, error::TreasuryError};

#[derive(Debug, Clone, PartialEq)]
pub struct Money {
    pub amount: BigDecimal,
    pub currency: CurrencyId,
}

impl Money {
    pub fn positive(amount: BigDecimal, currency: CurrencyId) -> Result<Self, TreasuryError> {
        if amount <= BigDecimal::zero() {
            return Err(TreasuryError::InvalidAmount);
        }
        Ok(Self { amount, currency })
    }

    pub fn non_negative(amount: BigDecimal, currency: CurrencyId) -> Result<Self, TreasuryError> {
        if amount < BigDecimal::zero() {
            return Err(TreasuryError::InvalidAmount);
        }
        Ok(Self { amount, currency })
    }

    pub fn zero(currency: CurrencyId) -> Self {
        Self {
            amount: BigDecimal::zero(),
            currency,
        }
    }

    pub fn add(&self, other: &Money) -> Result<Money, TreasuryError> {
        self.ensure_same_currency(other)?;
        Ok(Money {
            amount: &self.amount + &other.amount,
            currency: self.currency,
        })
    }

    pub fn subtract(&self, other: &Money) -> Result<Money, TreasuryError> {
        self.ensure_same_currency(other)?;
        let new_amount = &self.amount - &other.amount;
        if new_amount < BigDecimal::zero() {
            return Err(TreasuryError::InsufficientFunds);
        }
        Ok(Money {
            amount: new_amount,
            currency: self.currency,
        })
    }

    pub fn has_enough(&self, required: &Money) -> Result<bool, TreasuryError> {
        self.ensure_same_currency(required)?;
        Ok(self.amount >= required.amount)
    }

    fn ensure_same_currency(&self, other: &Money) -> Result<(), TreasuryError> {
        if self.currency != other.currency {
            Err(TreasuryError::CurrencyMismatch)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bigdecimal::FromPrimitive;
    use uuid::Uuid;

    fn currency() -> CurrencyId {
        CurrencyId(Uuid::new_v4())
    }

    fn money(amount: u64, currency: CurrencyId) -> Money {
        Money {
            amount: BigDecimal::from_u64(amount).unwrap(),
            currency,
        }
    }

    #[test]
    fn positive_rejects_zero() {
        let c = currency();
        let result = Money::positive(BigDecimal::zero(), c);
        assert!(matches!(result, Err(TreasuryError::InvalidAmount)));
    }

    #[test]
    fn add_same_currency_succeeds() {
        let c = currency();
        let total = money(10, c).add(&money(5, c)).unwrap();
        assert_eq!(total.amount, BigDecimal::from_u64(15).unwrap());
    }

    #[test]
    fn add_different_currency_fails() {
        let result = money(10, currency()).add(&money(5, currency()));
        assert!(matches!(result, Err(TreasuryError::CurrencyMismatch)));
    }

    #[test]
    fn subtract_below_zero_fails() {
        let c = currency();
        let result = money(5, c).subtract(&money(10, c));
        assert!(matches!(result, Err(TreasuryError::InsufficientFunds)));
    }
}
