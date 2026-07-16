use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;

use super::types::InvestmentStrategyId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValuationMode {
    Simulated,
    MarkToMarket,
}

impl ValuationMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Simulated => "simulated",
            Self::MarkToMarket => "mark_to_market",
        }
    }

    pub fn parse(s: &str) -> Self {
        match s {
            "mark_to_market" => Self::MarkToMarket,
            _ => Self::Simulated,
        }
    }
}

#[derive(Debug, Clone)]
pub struct InvestmentStrategy {
    pub id: InvestmentStrategyId,
    pub name: String,
    pub description: String,
    pub risk_level: String,
    pub expected_return_percentage: BigDecimal,
    pub duration_days: i32,
    pub valuation_mode: ValuationMode,
    pub category: String,
    pub ragequit_fee_bps: i32,
    pub created_at: NaiveDateTime,
}
