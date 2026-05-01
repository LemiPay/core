use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExpenseId(pub Uuid);

impl ExpenseId {
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Display for ExpenseId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
