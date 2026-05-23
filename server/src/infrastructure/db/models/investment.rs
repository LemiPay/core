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
}

impl From<InvestmentStatusModel> for InvestmentStatus {
    fn from(value: InvestmentStatusModel) -> Self {
        match value {
            InvestmentStatusModel::Active => InvestmentStatus::Active,
            InvestmentStatusModel::Matured => InvestmentStatus::Matured,
            InvestmentStatusModel::Withdrawn => InvestmentStatus::Withdrawn,
        }
    }
}

impl From<InvestmentStatus> for InvestmentStatusModel {
    fn from(value: InvestmentStatus) -> Self {
        match value {
            InvestmentStatus::Active => InvestmentStatusModel::Active,
            InvestmentStatus::Matured => InvestmentStatusModel::Matured,
            InvestmentStatus::Withdrawn => InvestmentStatusModel::Withdrawn,
        }
    }
}

// Strategy

#[derive(Queryable, Selectable, Debug)]
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
    pub expected_return: BigDecimal,
    pub actual_return: Option<BigDecimal>,
    pub status: InvestmentStatusModel,
    pub started_at: NaiveDateTime,
    pub matures_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = schema::investment)]
pub struct NewInvestmentModel {
    pub id: Uuid,
    pub proposal_id: Uuid,
    pub amount: BigDecimal,
    pub expected_return: BigDecimal,
    pub status: InvestmentStatusModel,
    pub started_at: NaiveDateTime,
    pub matures_at: NaiveDateTime,
}
