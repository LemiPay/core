use crate::domain::governance::status::ProposalStatus;

/// Voting policy is intentionally pluggable so we can swap the bootstrap
/// behavior for a real voting system without touching use cases.
pub trait VotingPolicy {
    fn initial_status(&self) -> ProposalStatus;
}

/// Bootstrap policy that auto-approves every proposal at creation time.
/// Mirrors the behavior in `/old`. Replace with a quorum-based policy when
/// the voting feature is implemented.
#[derive(Debug, Clone, Copy, Default)]
pub struct AutoApproveVotingPolicy;

impl VotingPolicy for AutoApproveVotingPolicy {
    fn initial_status(&self) -> ProposalStatus {
        ProposalStatus::Approved
    }
}
