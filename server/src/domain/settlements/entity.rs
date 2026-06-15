use crate::domain::user::UserId;
use bigdecimal::BigDecimal;

pub struct Settlement {
    pub from: UserId,
    pub to: UserId,
    pub amount: BigDecimal,
}
