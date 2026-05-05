use std::fmt::{Display, Formatter};
use uuid::Uuid;

use crate::domain::{
    treasury::{currency::CurrencyId, error::TreasuryError, money::Money},
    user::UserId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UserWalletId(pub Uuid);

impl UserWalletId {
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Display for UserWalletId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct UserWallet {
    pub id: UserWalletId,
    pub address: String,
    pub user_id: UserId,
    pub balance: Money,
}

impl UserWallet {
    pub fn new(
        address: String,
        user_id: UserId,
        currency: CurrencyId,
    ) -> Result<Self, TreasuryError> {
        let trimmed = address.trim();
        if trimmed.is_empty() {
            return Err(TreasuryError::InvalidAddress);
        }
        Ok(Self {
            id: UserWalletId(Uuid::new_v4()),
            address: trimmed.to_string(),
            user_id,
            balance: Money::zero(currency),
        })
    }

    pub fn rehydrate(id: UserWalletId, address: String, user_id: UserId, balance: Money) -> Self {
        Self {
            id,
            address,
            user_id,
            balance,
        }
    }

    pub fn deposit(self, amount: &Money) -> Result<Self, TreasuryError> {
        let new_balance = self.balance.add(amount)?;
        Ok(Self {
            balance: new_balance,
            ..self
        })
    }

    pub fn withdraw(self, amount: &Money) -> Result<Self, TreasuryError> {
        let new_balance = self.balance.subtract(amount)?;
        Ok(Self {
            balance: new_balance,
            ..self
        })
    }

    pub fn is_owned_by(&self, user_id: UserId) -> bool {
        self.user_id == user_id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bigdecimal::{BigDecimal, FromPrimitive};
    use uuid::Uuid;

    fn fixture() -> (UserWallet, CurrencyId) {
        let currency = CurrencyId(Uuid::new_v4());
        let wallet = UserWallet::new("addr".into(), UserId(Uuid::new_v4()), currency).unwrap();
        (wallet, currency)
    }

    #[test]
    fn new_wallet_starts_at_zero() {
        let (wallet, _) = fixture();
        assert_eq!(wallet.balance.amount, BigDecimal::from_u64(0).unwrap());
    }

    #[test]
    fn empty_address_is_rejected() {
        let result = UserWallet::new(
            "   ".into(),
            UserId(Uuid::new_v4()),
            CurrencyId(Uuid::new_v4()),
        );
        assert!(matches!(result, Err(TreasuryError::InvalidAddress)));
    }

    #[test]
    fn deposit_increases_balance() {
        let (wallet, currency) = fixture();
        let amount = Money::positive(BigDecimal::from_u64(50).unwrap(), currency).unwrap();
        let updated = wallet.deposit(&amount).unwrap();
        assert_eq!(updated.balance.amount, BigDecimal::from_u64(50).unwrap());
    }

    #[test]
    fn withdraw_without_funds_fails() {
        let (wallet, currency) = fixture();
        let amount = Money::positive(BigDecimal::from_u64(50).unwrap(), currency).unwrap();
        let result = wallet.withdraw(&amount);
        assert!(matches!(result, Err(TreasuryError::InsufficientFunds)));
    }
}
