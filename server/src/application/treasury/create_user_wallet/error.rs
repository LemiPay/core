use crate::domain::treasury::TreasuryError;

#[derive(Debug)]
pub enum CreateUserWalletError {
    InvalidAddress,
    CurrencyNotFound,
    AddressTakenByOtherUser,
    AddressAlreadyHasCurrency,
    Internal,
}

impl From<TreasuryError> for CreateUserWalletError {
    fn from(err: TreasuryError) -> Self {
        match err {
            TreasuryError::InvalidAddress => CreateUserWalletError::InvalidAddress,
            _ => CreateUserWalletError::Internal,
        }
    }
}
