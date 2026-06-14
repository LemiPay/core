use crate::application::{
    auth::AuthService, balances::BalancesService, expense::ExpenseService,
    governance::GovernanceService, group::GroupService, investment::InvestmentService,
    treasury::TreasuryService, users::UserService,
};
use crate::infrastructure::db::repositories::notifications_repo_impl::DieselNotificationRepository;
use crate::infrastructure::email::email_sender::EmailService;
use std::sync::Arc;

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
    pub notification_repo: Arc<DieselNotificationRepository>,
    pub email_service: Arc<dyn EmailService>,
    pub notification_service: Arc<crate::application::notifications::NotificationService>,
}

pub type SharedState = Arc<AppState>;
