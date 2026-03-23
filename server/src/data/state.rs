use crate::services::auth::AuthService;
use crate::services::user::UserService;
use std::sync::Arc;

// ----------------------
// APP STATE
// ----------------------

#[derive(Clone)]
pub struct AppState {
    pub user_service: UserService,
    pub auth_service: AuthService,
}

pub type SharedState = Arc<AppState>;
