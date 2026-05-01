use crate::application::common::repo_error::RepoError;

#[derive(Debug)]
pub enum BalancesError {
    Internal,
    UserNotFound,
    InsufficientFunds,
}

impl From<RepoError> for BalancesError {
    fn from(_: RepoError) -> Self {
        BalancesError::Internal
    }
}

impl From<crate::domain::balances::BalancesError> for BalancesError {
    fn from(value: crate::domain::balances::BalancesError) -> Self {
        match value {
            crate::domain::balances::BalancesError::UserNotFound => BalancesError::UserNotFound,
            crate::domain::balances::BalancesError::InsufficientFunds => {
                BalancesError::InsufficientFunds
            }
        }
    }
}
