use crate::application::common::repo_error::RepoError;
use crate::domain::governance::GovernanceError as DomainGovernanceError;

#[derive(Debug)]
pub enum GovernanceError {
    NotFound,
    Internal,
    InvalidAmount,
    InvalidEmail,
    MissingProposalTarget,
    UserNotFound,
    UserAlreadyMember,
    NotProposalDestination,
    NotProposalCreator,
    ProposalAlreadyResolved,
    GroupMismatch,
    FundRoundNotActive,
    ContributionExceedsTarget,
    InvalidStatusTransition,
    SenderWalletNotFound,
}

impl From<RepoError> for GovernanceError {
    fn from(_: RepoError) -> Self {
        GovernanceError::Internal
    }
}

impl From<DomainGovernanceError> for GovernanceError {
    fn from(value: DomainGovernanceError) -> Self {
        match value {
            DomainGovernanceError::InvalidAmount => GovernanceError::InvalidAmount,
            DomainGovernanceError::InvalidEmail => GovernanceError::InvalidEmail,
            DomainGovernanceError::InvalidStatusTransition => {
                GovernanceError::InvalidStatusTransition
            }
            DomainGovernanceError::ProposalAlreadyResolved => {
                GovernanceError::ProposalAlreadyResolved
            }
            DomainGovernanceError::NotProposalDestination => {
                GovernanceError::NotProposalDestination
            }
            DomainGovernanceError::NotProposalCreator => GovernanceError::NotProposalCreator,
            DomainGovernanceError::GroupMismatch => GovernanceError::GroupMismatch,
            DomainGovernanceError::ContributionExceedsTarget => {
                GovernanceError::ContributionExceedsTarget
            }
            DomainGovernanceError::FundRoundNotActive => GovernanceError::FundRoundNotActive,
            DomainGovernanceError::MissingProposalTarget => GovernanceError::MissingProposalTarget,
            DomainGovernanceError::UserAlreadyMember => GovernanceError::UserAlreadyMember,
            DomainGovernanceError::UserNotFound => GovernanceError::UserNotFound,
        }
    }
}
