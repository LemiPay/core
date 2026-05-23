use bigdecimal::BigDecimal;
use chrono::{NaiveDateTime, TimeDelta};

use super::{
    status::InvestmentStatus,
    types::{InvestmentId, InvestmentStrategyId},
};
use crate::domain::{governance::ProposalId, group::GroupId, treasury::CurrencyId};

#[derive(Debug, Clone)]
pub struct Investment {
    pub id: InvestmentId,
    pub group_id: GroupId,
    pub proposal_id: ProposalId,
    pub strategy_id: InvestmentStrategyId,
    pub currency_id: CurrencyId,
    pub amount: BigDecimal,
    pub current_value: BigDecimal,
    pub actual_return: Option<BigDecimal>,
    pub status: InvestmentStatus,
    pub started_at: NaiveDateTime,
    pub matures_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl Investment {
    pub fn rehydrate(
        id: InvestmentId,
        group_id: GroupId,
        proposal_id: ProposalId,
        strategy_id: InvestmentStrategyId,
        currency_id: CurrencyId,
        amount: BigDecimal,
        current_value: BigDecimal,
        actual_return: Option<BigDecimal>,
        status: InvestmentStatus,
        started_at: NaiveDateTime,
        matures_at: NaiveDateTime,
        created_at: NaiveDateTime,
        updated_at: NaiveDateTime,
    ) -> Self {
        Self {
            id,
            group_id,
            proposal_id,
            strategy_id,
            currency_id,
            amount,
            current_value,
            actual_return,
            status,
            started_at,
            matures_at,
            created_at,
            updated_at,
        }
    }

    pub fn calculate_matures_at(started_at: NaiveDateTime, duration_days: i32) -> NaiveDateTime {
        started_at + TimeDelta::days(duration_days as i64)
    }
}
