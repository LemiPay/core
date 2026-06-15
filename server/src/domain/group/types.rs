use crate::id_type;

id_type!(GroupId);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GroupStatus {
    Active,
    Ended,
    DebtResolution,
}
