use std::fmt::{Display, Formatter};
use uuid::Uuid;

use crate::domain::{
    group::GroupId,
    treasury::{currency::CurrencyId, error::TreasuryError, money::Money},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GroupWalletId(pub Uuid);

impl GroupWalletId {
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Display for GroupWalletId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone)]
pub struct GroupWallet {
    pub id: GroupWalletId,
    pub address: String,
    pub group_id: GroupId,
    pub balance: Money,
}

impl GroupWallet {
    pub fn new(
        address: String,
        group_id: GroupId,
        currency: CurrencyId,
    ) -> Result<Self, TreasuryError> {
        let trimmed = address.trim();
        if trimmed.is_empty() {
            return Err(TreasuryError::InvalidAddress);
        }
        Ok(Self {
            id: GroupWalletId(Uuid::new_v4()),
            address: trimmed.to_string(),
            group_id,
            balance: Money::zero(currency),
        })
    }

    pub fn rehydrate(
        id: GroupWalletId,
        address: String,
        group_id: GroupId,
        balance: Money,
    ) -> Self {
        Self {
            id,
            address,
            group_id,
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
}
