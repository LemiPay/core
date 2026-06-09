use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InvestmentId(pub Uuid);

impl InvestmentId {
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Display for InvestmentId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InvestmentStrategyId(pub Uuid);

impl InvestmentStrategyId {
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Display for InvestmentStrategyId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
