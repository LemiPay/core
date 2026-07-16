use super::error::InvestmentError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvestmentStatus {
    Active,
    Matured,
    Withdrawn,
}

impl InvestmentStatus {
    pub fn is_terminal(self) -> bool {
        matches!(self, InvestmentStatus::Withdrawn)
    }

    pub fn can_transition_to(self, next: InvestmentStatus) -> bool {
        use InvestmentStatus::*;
        match (self, next) {
            (Active, Matured) => true,
            // maturity withdraw or ragequit (early exit)
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
    fn active_can_mature() {
        assert!(InvestmentStatus::Active.can_transition_to(InvestmentStatus::Matured));
    }

    #[test]
    fn matured_can_withdraw() {
        assert!(InvestmentStatus::Matured.can_transition_to(InvestmentStatus::Withdrawn));
    }

    #[test]
    fn active_can_ragequit() {
        assert!(InvestmentStatus::Active.can_transition_to(InvestmentStatus::Withdrawn));
    }

    #[test]
    fn withdrawn_is_terminal() {
        assert!(InvestmentStatus::Withdrawn.is_terminal());
        assert!(!InvestmentStatus::Withdrawn.can_transition_to(InvestmentStatus::Active));
    }
}
