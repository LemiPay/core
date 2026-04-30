use crate::application::{auth::AuthService, group::GroupService, users::UserService};
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
}

pub type SharedState = Arc<AppState>;
