use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;

use super::types::InvestmentStrategyId;

#[derive(Debug, Clone)]
pub struct InvestmentStrategy {
    pub id: InvestmentStrategyId,
    pub name: String,
    pub description: String,
    pub risk_level: String,
    pub expected_return_percentage: BigDecimal,
    pub duration_days: i32,
    pub created_at: NaiveDateTime,
}
