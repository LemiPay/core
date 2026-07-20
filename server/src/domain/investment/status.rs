use super::error::InvestmentError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvestmentStatus {
    Active,
    Matured,
    Withdrawn,
    /// Margin burned; cannot withdraw.
    Liquidated,
}

impl InvestmentStatus {
    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            InvestmentStatus::Withdrawn | InvestmentStatus::Liquidated
        )
    }

    pub fn can_transition_to(self, next: InvestmentStatus) -> bool {
        use InvestmentStatus::*;
        match (self, next) {
            (Active, Matured) => true,
            (Active, Liquidated) => true,
            (Matured, Withdrawn) | (Active, Withdrawn) => true,
            _ => false,
        }
    }

    pub fn ensure_can_transition_to(self, next: InvestmentStatus) -> Result<(), InvestmentError> {
        if self.can_transition_to(next) {
            Ok(())
        } else {
            Err(InvestmentError::InvalidStatusTransition)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn active_can_liquidate() {
        assert!(InvestmentStatus::Active.can_transition_to(InvestmentStatus::Liquidated));
    }

    #[test]
    fn liquidated_is_terminal() {
        assert!(InvestmentStatus::Liquidated.is_terminal());
        assert!(!InvestmentStatus::Liquidated.can_transition_to(InvestmentStatus::Withdrawn));
    }
}
