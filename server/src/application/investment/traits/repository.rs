use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::application::common::repo_error::RepoError;
use crate::application::investment::dto::{
    ActiveInvestmentDto, AssetPriceDto, InvestmentDetails, InvestmentProposalDetails,
    InvestmentStrategyDto, NewHolding, SnapshotDto,
};
use crate::domain::investment::member::NewInvestmentMember;

pub trait InvestmentRepository: Send + Sync {
    // Strategies
    fn list_strategies(&self) -> Result<Vec<InvestmentStrategyDto>, RepoError>;
    fn find_strategy(&self, strategy_id: Uuid) -> Result<Option<InvestmentStrategyDto>, RepoError>;
    fn list_strategy_assets_for_pricing(
        &self,
        strategy_id: Uuid,
    ) -> Result<Vec<AssetPriceDto>, RepoError>;

    // Investment proposals
    fn list_approved_proposals(
        &self,
        group_id: Uuid,
    ) -> Result<Vec<InvestmentProposalDetails>, RepoError>;
    fn create_investment_proposal(
        &self,
        created_by: Uuid,
        group_id: Uuid,
        amount: BigDecimal,
        strategy_id: Uuid,
        currency_id: Uuid,
    ) -> Result<InvestmentProposalDetails, RepoError>;
    fn find_investment_proposal(
        &self,
        proposal_id: Uuid,
    ) -> Result<Option<InvestmentProposalDetails>, RepoError>;

    // Investments
    fn execute_investment(
        &self,
        proposal_id: Uuid,
        group_id: Uuid,
        user_id: Uuid,
        amount: BigDecimal,
        entry_exposure: BigDecimal,
        strategy_id: Uuid,
        currency_id: Uuid,
        matures_at: chrono::NaiveDateTime,
        participants: Vec<NewInvestmentMember>,
        holdings: Vec<NewHolding>,
    ) -> Result<InvestmentDetails, RepoError>;
    fn find_investment(&self, investment_id: Uuid) -> Result<Option<InvestmentDetails>, RepoError>;
    fn list_group_investments(&self, group_id: Uuid) -> Result<Vec<InvestmentDetails>, RepoError>;
    /// Credits `payout` to group wallet, marks withdrawn with exit metadata.
    fn withdraw_investment(
        &self,
        investment_id: Uuid,
        group_id: Uuid,
        user_id: Uuid,
        payout: BigDecimal,
        actual_return: BigDecimal,
        exit_kind: &str,
        fee_amount: BigDecimal,
    ) -> Result<InvestmentDetails, RepoError>;
    // Snapshots
    fn list_snapshots(&self, investment_id: Uuid) -> Result<Vec<SnapshotDto>, RepoError>;

    // ── Pulse (background value simulation / MTM) ──
    fn list_active_with_strategy(&self) -> Result<Vec<ActiveInvestmentDto>, RepoError>;
    fn list_holdings_for_pricing(
        &self,
        investment_id: Uuid,
    ) -> Result<Vec<(AssetPriceDto, BigDecimal)>, RepoError>;
    fn count_snapshots(&self, investment_id: Uuid) -> Result<i64, RepoError>;
    fn update_current_value(
        &self,
        investment_id: Uuid,
        value: BigDecimal,
        now: NaiveDateTime,
    ) -> Result<(), RepoError>;
    fn mature_investment(
        &self,
        investment_id: Uuid,
        final_value: BigDecimal,
        actual_return: BigDecimal,
        now: NaiveDateTime,
    ) -> Result<(), RepoError>;
    /// Burn margin: status liquidated, current_value=0, no wallet credit.
    fn liquidate_investment(
        &self,
        investment_id: Uuid,
        margin: BigDecimal,
        now: NaiveDateTime,
    ) -> Result<(), RepoError>;
    fn insert_snapshot(
        &self,
        investment_id: Uuid,
        value: BigDecimal,
        snapshot_date: NaiveDateTime,
    ) -> Result<(), RepoError>;
}
