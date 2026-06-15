use std::sync::Arc;

use crate::application::{
    auth::AuthService, balances::BalancesService, expense::ExpenseService,
    governance::GovernanceService, group::GroupService, investment::InvestmentService,
    treasury::TreasuryService, treasury::traits::currency_repo::CurrencyRepository,
    treasury::traits::fund_event_repo::FundEventRepository, users::UserService,
};
use crate::infrastructure::blockchain::BlockchainService;

use super::config::AppConfig;

// ----------------------
// APP STATE
// ----------------------

pub struct AppState {
    pub config: AppConfig,

    pub auth_service: AuthService,
    pub user_service: UserService,
    pub group_service: GroupService,
    pub treasury_service: TreasuryService,
    pub governance_service: GovernanceService,
    pub expense_service: ExpenseService,
    pub balances_service: BalancesService,
    pub investment_service: InvestmentService,

    pub blockchain_service: Arc<dyn BlockchainService>,
    pub fund_event_repo: Arc<dyn FundEventRepository>,
    pub currency_repo: Arc<dyn CurrencyRepository>,
}

pub type SharedState = Arc<AppState>;
