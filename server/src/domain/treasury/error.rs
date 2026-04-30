#[derive(Debug)]
pub enum TreasuryError {
    InvalidAmount,
    InvalidAddress,
    InsufficientFunds,
    CurrencyMismatch,
    SameWalletTransfer,
}
