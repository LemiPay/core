use crate::domain::governance::error::GovernanceError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProposalStatus {
    Pending,
    Approved,
    Rejected,
    Executed,
    Canceled,
    Expired,
    Failed,
}

impl ProposalStatus {
    pub fn is_open(self) -> bool {
        matches!(self, ProposalStatus::Pending | ProposalStatus::Approved)
    }

    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            ProposalStatus::Executed
                | ProposalStatus::Rejected
                | ProposalStatus::Canceled
                | ProposalStatus::Expired
                | ProposalStatus::Failed
        )
    }

    pub fn can_transition_to(self, next: ProposalStatus) -> bool {
        use ProposalStatus::*;
        match (self, next) {
            (Pending, Approved | Rejected | Canceled | Expired) => true,
            (Approved, Executed | Canceled | Expired | Failed | Rejected) => true,
            _ => false,
        }
    }

    pub fn ensure_can_transition_to(self, next: ProposalStatus) -> Result<(), GovernanceError> {
        if self.can_transition_to(next) {
            Ok(())
        } else {
            Err(GovernanceError::InvalidStatusTransition)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn approved_can_transition_to_executed() {
        assert!(ProposalStatus::Approved.can_transition_to(ProposalStatus::Executed));
    }

    #[test]
    fn executed_is_terminal() {
        assert!(ProposalStatus::Executed.is_terminal());
        assert!(!ProposalStatus::Executed.can_transition_to(ProposalStatus::Approved));
    }

    #[test]
    fn pending_can_transition_to_approved_or_rejected() {
        assert!(ProposalStatus::Pending.can_transition_to(ProposalStatus::Approved));
        assert!(ProposalStatus::Pending.can_transition_to(ProposalStatus::Rejected));
    }
}
