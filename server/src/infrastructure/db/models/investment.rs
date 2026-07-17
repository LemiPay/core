use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use diesel::{Insertable, Queryable, Selectable};
use diesel_derive_enum::DbEnum;
use serde::Serialize;
use uuid::Uuid;

use crate::domain::investment::InvestmentStatus;
use crate::infrastructure::db::schema;

#[derive(Debug, DbEnum, Clone, Copy, PartialEq, Eq, Serialize)]
#[db_enum(existing_type_path = "crate::infrastructure::db::schema::sql_types::InvestmentStatus")]
pub enum InvestmentStatusModel {
    Active,
    Matured,
    Withdrawn,
    Liquidated,
}

impl From<InvestmentStatusModel> for InvestmentStatus {
    fn from(value: InvestmentStatusModel) -> Self {
        match value {
            InvestmentStatusModel::Active => InvestmentStatus::Active,
            InvestmentStatusModel::Matured => InvestmentStatus::Matured,
            InvestmentStatusModel::Withdrawn => InvestmentStatus::Withdrawn,
            InvestmentStatusModel::Liquidated => InvestmentStatus::Liquidated,
        }
    }
}

impl From<InvestmentStatus> for InvestmentStatusModel {
    fn from(value: InvestmentStatus) -> Self {
        match value {
            InvestmentStatus::Active => InvestmentStatusModel::Active,
            InvestmentStatus::Matured => InvestmentStatusModel::Matured,
            InvestmentStatus::Withdrawn => InvestmentStatusModel::Withdrawn,
            InvestmentStatus::Liquidated => InvestmentStatusModel::Liquidated,
        }
    }
}

// Strategy

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = schema::investment_strategy)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InvestmentStrategyModel {
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
}

// Asset

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = schema::asset)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct AssetModel {
    pub id: Uuid,
    pub symbol: String,
    pub name: String,
    pub kind: String,
    pub price_provider: String,
    pub external_id: String,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
}

// Strategy allocation

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = schema::strategy_allocation)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct StrategyAllocationModel {
    pub id: Uuid,
    pub strategy_id: Uuid,
    pub asset_id: Uuid,
    pub weight_bps: i32,
}

// Investment holding

#[derive(Queryable, Selectable, Debug, Clone)]
#[diesel(table_name = schema::investment_holding)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InvestmentHoldingModel {
    pub id: Uuid,
    pub investment_id: Uuid,
    pub asset_id: Uuid,
    pub units: BigDecimal,
    pub weight_bps_at_entry: i32,
    pub cost_basis_usd: BigDecimal,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = schema::investment_holding)]
pub struct NewInvestmentHoldingModel {
    pub investment_id: Uuid,
    pub asset_id: Uuid,
    pub units: BigDecimal,
    pub weight_bps_at_entry: i32,
    pub cost_basis_usd: BigDecimal,
}

// Investment Proposal

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = schema::investment_proposal)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InvestmentProposalModel {
    pub proposal_id: Uuid,
    pub amount: BigDecimal,
    pub strategy_id: Uuid,
    pub currency_id: Uuid,
}

#[derive(Insertable)]
#[diesel(table_name = schema::investment_proposal)]
pub struct NewInvestmentProposalModel {
    pub proposal_id: Uuid,
    pub amount: BigDecimal,
    pub strategy_id: Uuid,
    pub currency_id: Uuid,
}

// Investment

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = schema::investment)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InvestmentModel {
    pub id: Uuid,
    pub proposal_id: Uuid,
    pub amount: BigDecimal,
    pub current_value: BigDecimal,
    pub actual_return: Option<BigDecimal>,
    pub status: InvestmentStatusModel,
    pub started_at: NaiveDateTime,
    pub matures_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub exit_kind: Option<String>,
    pub fee_amount: Option<BigDecimal>,
    pub entry_exposure: BigDecimal,
}

#[derive(Insertable)]
#[diesel(table_name = schema::investment)]
pub struct NewInvestmentModel {
    pub id: Uuid,
    pub proposal_id: Uuid,
    pub amount: BigDecimal,
    pub current_value: BigDecimal,
    pub status: InvestmentStatusModel,
    pub started_at: NaiveDateTime,
    pub matures_at: NaiveDateTime,
    pub entry_exposure: BigDecimal,
}

// Investment Member

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = schema::investment_member)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InvestmentMemberModel {
    pub id: Uuid,
    pub investment_id: Uuid,
    pub user_id: Uuid,
    pub balance_at_investment: BigDecimal,
    pub participation_pct: BigDecimal,
    pub invested_amount: BigDecimal,
    pub returned_amount: Option<BigDecimal>,
    pub withdrawn_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = schema::investment_member)]
pub struct NewInvestmentMemberModel {
    pub investment_id: Uuid,
    pub user_id: Uuid,
    pub balance_at_investment: BigDecimal,
    pub participation_pct: BigDecimal,
    pub invested_amount: BigDecimal,
    pub returned_amount: Option<BigDecimal>,
    pub withdrawn_at: Option<NaiveDateTime>,
}

// Investment Value Snapshot

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = schema::investment_value_snapshot)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct InvestmentValueSnapshotModel {
    pub id: Uuid,
    pub investment_id: Uuid,
    pub value: BigDecimal,
    pub snapshot_date: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = schema::investment_value_snapshot)]
pub struct NewInvestmentValueSnapshotModel {
    pub investment_id: Uuid,
    pub value: BigDecimal,
    pub snapshot_date: NaiveDateTime,
}
