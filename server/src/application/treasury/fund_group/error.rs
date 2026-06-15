use crate::domain::treasury::TreasuryError;

#[derive(Debug)]
pub enum FundGroupError {
    InvalidAmount,
    UserWalletNotFound,
    GroupWalletNotFound,
    InsufficientFunds,
    GroupNotActive,
    GroupNotFound,
    Internal,
}

impl From<TreasuryError> for FundGroupError {
    fn from(err: TreasuryError) -> Self {
        match err {
            TreasuryError::InvalidAmount => FundGroupError::InvalidAmount,
            TreasuryError::InsufficientFunds => FundGroupError::InsufficientFunds,
            _ => FundGroupError::Internal,
        }
    }
}
