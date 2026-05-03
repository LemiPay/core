#[derive(Debug, PartialEq, Eq)]
pub enum BalancesError {
    UserNotFound,
    InsufficientFunds,
}
