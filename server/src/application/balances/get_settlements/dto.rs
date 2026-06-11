use crate::domain::group::GroupId;
use crate::domain::settlements::entity::Settlement;
use crate::domain::user::{UserId, UserName};
use bigdecimal::BigDecimal;

pub struct GetSettlementsInput {
    pub group_id: GroupId,
}

pub struct GetSettlementsOutput {
    pub settlements: Vec<SettlementDetails>,
}

pub struct SettlementDetails {
    pub from: UserId,
    pub to: UserId,
    pub amount: BigDecimal,
    pub to_name: Option<UserName>,
    pub from_name: Option<UserName>,
}
