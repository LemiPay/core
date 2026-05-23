use bigdecimal::BigDecimal;
use uuid::Uuid;

use crate::application::common::repo_error::RepoError;
use crate::application::investment::dto::{
    InvestmentDetails, InvestmentProposalDetails, InvestmentStrategyDto,
};

pub trait InvestmentRepository: Send + Sync {
    // Strategies
    fn list_strategies(&self) -> Result<Vec<InvestmentStrategyDto>, RepoError>;
    fn find_strategy(&self, strategy_id: Uuid) -> Result<Option<InvestmentStrategyDto>, RepoError>;

    // Investment proposals
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
        strategy_id: Uuid,
        currency_id: Uuid,
        expected_return: BigDecimal,
        matures_at: chrono::NaiveDateTime,
    ) -> Result<InvestmentDetails, RepoError>;
    fn find_investment(&self, investment_id: Uuid) -> Result<Option<InvestmentDetails>, RepoError>;
    fn list_group_investments(&self, group_id: Uuid) -> Result<Vec<InvestmentDetails>, RepoError>;
    fn mature_investment(
        &self,
        investment_id: Uuid,
        actual_return: BigDecimal,
    ) -> Result<InvestmentDetails, RepoError>;
    fn withdraw_investment(
        &self,
        investment_id: Uuid,
        group_id: Uuid,
        user_id: Uuid,
    ) -> Result<InvestmentDetails, RepoError>;
    fn list_maturable_investments(
        &self,
        now: chrono::NaiveDateTime,
    ) -> Result<Vec<InvestmentDetails>, RepoError>;
}
