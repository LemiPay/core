use crate::domain::treasury::TreasuryError;

#[derive(Debug)]
pub enum TransferFundsError {
    InvalidAmount,
    SenderWalletNotFound,
    NotOwner,
    InsufficientFunds,
    SameWalletTransfer,
    ReceiverNotFound,
    Internal,
}

impl From<TreasuryError> for TransferFundsError {
    fn from(err: TreasuryError) -> Self {
        match err {
            TreasuryError::InvalidAmount => TransferFundsError::InvalidAmount,
            TreasuryError::InsufficientFunds => TransferFundsError::InsufficientFunds,
            TreasuryError::SameWalletTransfer => TransferFundsError::SameWalletTransfer,
            TreasuryError::CurrencyMismatch | TreasuryError::InvalidAddress => {
                TransferFundsError::Internal
            }
        }
    }
}
