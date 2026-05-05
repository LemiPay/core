use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CurrencyId(pub Uuid);

impl CurrencyId {
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Display for CurrencyId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
