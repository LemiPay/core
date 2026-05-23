use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::domain::investment::InvestmentStatus;

pub struct InvestmentStrategyDto {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub risk_level: String,
    pub expected_return_percentage: BigDecimal,
    pub duration_days: i32,
    pub created_at: NaiveDateTime,
}

pub struct InvestmentProposalDetails {
    pub proposal_id: Uuid,
    pub group_id: Uuid,
    pub created_by: Uuid,
    pub status: InvestmentStatus,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub amount: BigDecimal,
    pub strategy_id: Uuid,
    pub currency_id: Uuid,
}

pub struct InvestmentDetails {
    pub id: Uuid,
    pub group_id: Uuid,
    pub proposal_id: Uuid,
    pub strategy_id: Uuid,
    pub currency_id: Uuid,
    pub amount: BigDecimal,
    pub expected_return: BigDecimal,
    pub actual_return: Option<BigDecimal>,
    pub status: InvestmentStatus,
    pub started_at: NaiveDateTime,
    pub matures_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub strategy_name: String,
    pub risk_level: String,
    pub expected_return_percentage: BigDecimal,
}
