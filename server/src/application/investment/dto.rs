use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::domain::investment::InvestmentStatus;

pub struct ActiveInvestmentDto {
    pub id: Uuid,
    pub group_id: Uuid,
    pub amount: BigDecimal,
    pub entry_exposure: BigDecimal,
    pub expected_return_percentage: BigDecimal,
    pub risk_level: String,
    pub duration_days: i32,
    pub valuation_mode: String,
    pub strategy_id: Uuid,
    pub leverage: i32,
}

pub struct AllocationDto {
    pub asset_id: Uuid,
    pub symbol: String,
    pub name: String,
    pub kind: String,
    pub weight_bps: i32,
    pub price_provider: String,
    pub external_id: String,
    pub price_source_url: String,
}

pub struct InvestmentStrategyDto {
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
    pub allocations: Vec<AllocationDto>,
}

pub struct HoldingDto {
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
    pub entry_price_usd: Option<BigDecimal>,
    pub current_price_usd: Option<BigDecimal>,
    pub current_value_usd: Option<BigDecimal>,
}

/// Holding ready to insert at execute time.
pub struct NewHolding {
    pub asset_id: Uuid,
    pub units: BigDecimal,
    pub weight_bps_at_entry: i32,
    pub cost_basis_usd: BigDecimal,
}

pub struct AssetPriceDto {
    pub id: Uuid,
    pub symbol: String,
    pub price_provider: String,
    pub external_id: String,
}

pub struct PulseResult {
    pub updated: usize,
    pub matured: usize,
    pub liquidated: usize,
    pub matured_group_ids: Vec<Uuid>,
    pub liquidated_group_ids: Vec<Uuid>,
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
    pub strategy_name: String,
}

pub struct SnapshotDto {
    pub investment_id: Uuid,
    pub value: BigDecimal,
    pub snapshot_date: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

pub struct InvestmentDetails {
    pub id: Uuid,
    pub group_id: Uuid,
    pub proposal_id: Uuid,
    pub strategy_id: Uuid,
    pub currency_id: Uuid,
    pub amount: BigDecimal,
    pub current_value: BigDecimal,
    pub actual_return: Option<BigDecimal>,
    pub status: InvestmentStatus,
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
    pub holdings: Vec<HoldingDto>,
}
