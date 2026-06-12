use super::config::AppConfig;
use crate::application::settlements::service::SettlementsService;
use crate::application::{
    auth::AuthService, balances::BalancesService, expense::ExpenseService,
    governance::GovernanceService, group::GroupService, investment::InvestmentService,
    treasury::TreasuryService, users::UserService,
};
use std::sync::Arc;

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
    pub settlements_service: SettlementsService,
    pub investment_service: InvestmentService,
}

pub type SharedState = Arc<AppState>;
