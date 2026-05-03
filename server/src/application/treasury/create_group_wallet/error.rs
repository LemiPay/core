use crate::domain::treasury::TreasuryError;

#[derive(Debug)]
pub enum CreateGroupWalletError {
    InvalidAddress,
    CurrencyNotFound,
    GroupAlreadyHasCurrency,
    AddressAlreadyTaken,
    Internal,
}

impl From<TreasuryError> for CreateGroupWalletError {
    fn from(err: TreasuryError) -> Self {
        match err {
            TreasuryError::InvalidAddress => CreateGroupWalletError::InvalidAddress,
            _ => CreateGroupWalletError::Internal,
        }
    }
}
