use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::investment::dto::{
    InvestmentDetails, InvestmentProposalDetails, InvestmentStrategyDto, SnapshotDto,
};
use crate::domain::investment::InvestmentStatus;

#[derive(Deserialize)]
pub struct CreateInvestmentProposalRequest {
    pub amount: String,
    pub strategy_id: Uuid,
    pub currency_id: Uuid,
}

#[derive(Deserialize)]
pub struct ExecuteInvestmentRequest {
    pub proposal_id: Uuid,
}

#[derive(Deserialize)]
pub struct WithdrawInvestmentRequest {
    pub investment_id: Uuid,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InvestmentStatusResponse {
    Active,
    Matured,
    Withdrawn,
}

impl From<InvestmentStatus> for InvestmentStatusResponse {
    fn from(value: InvestmentStatus) -> Self {
        match value {
            InvestmentStatus::Active => Self::Active,
            InvestmentStatus::Matured => Self::Matured,
            InvestmentStatus::Withdrawn => Self::Withdrawn,
        }
    }
}

#[derive(Serialize)]
pub struct InvestmentStrategyResponse {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub risk_level: String,
    pub expected_return_percentage: BigDecimal,
    pub duration_days: i32,
    pub created_at: NaiveDateTime,
}

impl From<InvestmentStrategyDto> for InvestmentStrategyResponse {
    fn from(value: InvestmentStrategyDto) -> Self {
        Self {
            id: value.id,
            name: value.name,
            description: value.description,
            risk_level: value.risk_level,
            expected_return_percentage: value.expected_return_percentage,
            duration_days: value.duration_days,
            created_at: value.created_at,
        }
    }
}

#[derive(Serialize)]
pub struct InvestmentProposalResponse {
    pub proposal_id: Uuid,
    pub group_id: Uuid,
    pub created_by: Uuid,
    pub status: InvestmentStatusResponse,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub amount: BigDecimal,
    pub strategy_id: Uuid,
    pub currency_id: Uuid,
}

impl From<InvestmentProposalDetails> for InvestmentProposalResponse {
    fn from(value: InvestmentProposalDetails) -> Self {
        Self {
            proposal_id: value.proposal_id,
            group_id: value.group_id,
            created_by: value.created_by,
            status: value.status.into(),
            created_at: value.created_at,
            updated_at: value.updated_at,
            amount: value.amount,
            strategy_id: value.strategy_id,
            currency_id: value.currency_id,
        }
    }
}

#[derive(Serialize)]
pub struct InvestmentResponse {
    pub id: Uuid,
    pub group_id: Uuid,
    pub proposal_id: Uuid,
    pub strategy_id: Uuid,
    pub currency_id: Uuid,
    pub amount: BigDecimal,
    pub current_value: BigDecimal,
    pub actual_return: Option<BigDecimal>,
    pub status: InvestmentStatusResponse,
    pub started_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub strategy_name: String,
    pub risk_level: String,
    pub expected_return_percentage: BigDecimal,
}

impl From<InvestmentDetails> for InvestmentResponse {
    fn from(value: InvestmentDetails) -> Self {
        Self {
            id: value.id,
            group_id: value.group_id,
            proposal_id: value.proposal_id,
            strategy_id: value.strategy_id,
            currency_id: value.currency_id,
            amount: value.amount,
            current_value: value.current_value,
            actual_return: value.actual_return,
            status: value.status.into(),
            started_at: value.started_at,
            created_at: value.created_at,
            updated_at: value.updated_at,
            strategy_name: value.strategy_name,
            risk_level: value.risk_level,
            expected_return_percentage: value.expected_return_percentage,
        }
    }
}

#[derive(Serialize)]
pub struct SnapshotResponse {
    pub investment_id: Uuid,
    pub value: BigDecimal,
    pub snapshot_date: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

impl From<SnapshotDto> for SnapshotResponse {
    fn from(value: SnapshotDto) -> Self {
        Self {
            investment_id: value.investment_id,
            value: value.value,
            snapshot_date: value.snapshot_date,
            created_at: value.created_at,
        }
    }
}
