#[derive(Debug)]
pub enum GovernanceError {
    InvalidAmount,
    InvalidStatusTransition,
    ProposalAlreadyResolved,
    NotProposalDestination,
    NotProposalCreator,
    GroupMismatch,
    ContributionExceedsTarget,
    FundRoundNotActive,
    MissingProposalTarget,
    UserAlreadyMember,
    UserNotFound,
    InvalidEmail,
}
