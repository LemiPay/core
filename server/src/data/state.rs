use std::sync::Arc;
use crate::services::auth::AuthService;
use crate::services::user::UserService;

// ----------------------
// APP STATE
// ----------------------

#[derive(Clone)]
pub struct AppState {
    pub user_service: UserService,
    pub auth_service: AuthService
}

pub type SharedState = Arc<AppState>;