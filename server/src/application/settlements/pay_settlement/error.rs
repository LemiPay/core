use crate::application::balances::error::BalancesError;
use crate::domain::treasury::TreasuryError;

#[derive(Debug)]
pub enum PaySettlementError {
    InvalidAmount,
    UserWalletNotFound,
    GroupWalletNotFound,
    InsufficientFunds,
    GroupNotActive,
    GroupNotInDebtResolution,
    GroupNotFound,
    NoDebt,
    AmountExceedsDebt,
    Internal,
}

impl From<TreasuryError> for PaySettlementError {
    fn from(err: TreasuryError) -> Self {
        match err {
            TreasuryError::InvalidAmount => PaySettlementError::InvalidAmount,
            TreasuryError::InsufficientFunds => PaySettlementError::InsufficientFunds,
            _ => PaySettlementError::Internal,
        }
    }
}

impl From<BalancesError> for PaySettlementError {
    fn from(_: BalancesError) -> Self {
        PaySettlementError::Internal
    }
}
