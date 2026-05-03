use std::fmt::{Display, Formatter};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GroupId(pub Uuid);

impl GroupId {
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Display for GroupId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
