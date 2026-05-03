use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProposalId(pub Uuid);

impl ProposalId {
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Display for ProposalId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProposalKind {
    NewMember,
    Withdraw,
    FundRound,
}
