use crate::application::common::repo_error::RepoError;
use crate::domain::investment::InvestmentError as DomainInvestmentError;

#[derive(Debug)]
pub enum InvestmentError {
    NotFound,
    Internal,
    InvalidAmount,
    InvalidStatusTransition,
    NotMatured,
    AlreadyWithdrawn,
    StrategyNotFound,
    ProposalNotFound,
    GroupWalletNotFound,
    InsufficientGroupFunds,
    NotGroupAdmin,
    NotProposalCreator,
    NotGroupMember,
}

impl From<RepoError> for InvestmentError {
    fn from(_: RepoError) -> Self {
        InvestmentError::Internal
    }
}

impl From<DomainInvestmentError> for InvestmentError {
    fn from(value: DomainInvestmentError) -> Self {
        match value {
            DomainInvestmentError::InvalidAmount => InvestmentError::InvalidAmount,
            DomainInvestmentError::InvalidStatusTransition => {
                InvestmentError::InvalidStatusTransition
            }
            DomainInvestmentError::NotMatured => InvestmentError::NotMatured,
            DomainInvestmentError::AlreadyWithdrawn => InvestmentError::AlreadyWithdrawn,
            DomainInvestmentError::StrategyNotFound => InvestmentError::StrategyNotFound,
        }
    }
}
