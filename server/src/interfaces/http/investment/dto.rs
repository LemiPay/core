use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::investment::dto::{
    AllocationDto, HoldingDto, InvestmentDetails, InvestmentProposalDetails, InvestmentStrategyDto,
    SnapshotDto,
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
    Liquidated,
}

impl From<InvestmentStatus> for InvestmentStatusResponse {
    fn from(value: InvestmentStatus) -> Self {
        match value {
            InvestmentStatus::Active => Self::Active,
            InvestmentStatus::Matured => Self::Matured,
            InvestmentStatus::Withdrawn => Self::Withdrawn,
            InvestmentStatus::Liquidated => Self::Liquidated,
        }
    }
}

#[derive(Serialize)]
pub struct AllocationResponse {
    pub asset_id: Uuid,
    pub symbol: String,
    pub name: String,
    pub kind: String,
    pub weight_bps: i32,
    pub price_provider: String,
    pub external_id: String,
    pub price_source_url: String,
}

impl From<AllocationDto> for AllocationResponse {
    fn from(value: AllocationDto) -> Self {
        Self {
            asset_id: value.asset_id,
            symbol: value.symbol,
            name: value.name,
            kind: value.kind,
            weight_bps: value.weight_bps,
            price_provider: value.price_provider,
            external_id: value.external_id,
            price_source_url: value.price_source_url,
        }
    }
}

#[derive(Serialize)]
pub struct HoldingResponse {
    pub asset_id: Uuid,
    pub symbol: String,
    pub name: String,
    pub kind: String,
    pub units: BigDecimal,
    pub weight_bps_at_entry: i32,
    pub cost_basis_usd: BigDecimal,
    pub price_provider: String,
    pub external_id: String,
    pub price_source_url: String,
    /// Plain strings so the frontend always receives parseable numbers (never omitted).
    pub entry_price_usd: Option<String>,
    pub current_price_usd: Option<String>,
    pub current_value_usd: Option<String>,
}

fn bd_opt_to_string(v: Option<BigDecimal>) -> Option<String> {
    v.map(|d| d.normalized().to_string())
}

impl From<HoldingDto> for HoldingResponse {
    fn from(value: HoldingDto) -> Self {
        Self {
            asset_id: value.asset_id,
            symbol: value.symbol,
            name: value.name,
            kind: value.kind,
            units: value.units,
            weight_bps_at_entry: value.weight_bps_at_entry,
            cost_basis_usd: value.cost_basis_usd,
            price_provider: value.price_provider,
            external_id: value.external_id,
            price_source_url: value.price_source_url,
            entry_price_usd: bd_opt_to_string(value.entry_price_usd),
            current_price_usd: bd_opt_to_string(value.current_price_usd),
            current_value_usd: bd_opt_to_string(value.current_value_usd),
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
    pub valuation_mode: String,
    pub category: String,
    pub ragequit_fee_bps: i32,
    pub leverage: i32,
    pub allocations: Vec<AllocationResponse>,
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
            valuation_mode: value.valuation_mode,
            category: value.category,
            ragequit_fee_bps: value.ragequit_fee_bps,
            leverage: value.leverage,
            allocations: value.allocations.into_iter().map(Into::into).collect(),
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
    pub strategy_name: String,
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
            strategy_name: value.strategy_name,
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
    pub matures_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub strategy_name: String,
    pub risk_level: String,
    pub expected_return_percentage: BigDecimal,
    pub valuation_mode: String,
    pub category: String,
    pub ragequit_fee_bps: i32,
    pub leverage: i32,
    pub entry_exposure: BigDecimal,
    pub exit_kind: Option<String>,
    pub fee_amount: Option<BigDecimal>,
    pub holdings: Vec<HoldingResponse>,
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
            matures_at: value.matures_at,
            created_at: value.created_at,
            updated_at: value.updated_at,
            strategy_name: value.strategy_name,
            risk_level: value.risk_level,
            expected_return_percentage: value.expected_return_percentage,
            valuation_mode: value.valuation_mode,
            category: value.category,
            ragequit_fee_bps: value.ragequit_fee_bps,
            leverage: value.leverage,
            entry_exposure: value.entry_exposure,
            exit_kind: value.exit_kind,
            fee_amount: value.fee_amount,
            holdings: value.holdings.into_iter().map(Into::into).collect(),
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
