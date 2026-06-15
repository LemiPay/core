use crate::application::balances::error::BalancesError;
use crate::domain::group::error::GroupError;
use crate::domain::treasury::TreasuryError;

#[derive(Debug)]
pub enum ClaimError {
    InvalidAmount,
    UserWalletNotFound,
    GroupWalletNotFound,
    InsufficientFunds,
    GroupNotInDebtResolution,
    GroupNotFound,
    NoCredit,
    AmountExceedsCredit,
    Internal,
}

impl From<TreasuryError> for ClaimError {
    fn from(err: TreasuryError) -> Self {
        match err {
            TreasuryError::InvalidAmount => ClaimError::InvalidAmount,
            TreasuryError::InsufficientFunds => ClaimError::InsufficientFunds,
            _ => ClaimError::Internal,
        }
    }
}

impl From<BalancesError> for ClaimError {
    fn from(_: BalancesError) -> Self {
        ClaimError::Internal
    }
}

impl From<GroupError> for ClaimError {
    fn from(_: GroupError) -> Self {
        ClaimError::Internal
    }
}
