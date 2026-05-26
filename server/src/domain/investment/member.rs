use crate::domain::investment::{Investment, InvestmentId};
use crate::domain::user::UserId;
use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;

#[derive(Debug, Clone)]
pub struct NewInvestmentMember {
    pub user_id: UserId,
    pub balance_at_investment: BigDecimal,
    pub participation_pct: BigDecimal,
    pub invested_amount: BigDecimal,
    pub returned_amount: Option<BigDecimal>,
    pub withdrawn_at: Option<NaiveDateTime>,
}
