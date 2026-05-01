use crate::application::{
    auth::AuthService, governance::GovernanceService, group::GroupService,
    treasury::TreasuryService, users::UserService,
};
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
}

pub type SharedState = Arc<AppState>;
