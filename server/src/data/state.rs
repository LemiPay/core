use crate::data::database::Db;
use crate::services::auth::AuthService;
use crate::services::group::GroupService;
use crate::services::user::UserService;
use std::sync::Arc;
// ----------------------
// APP STATE
// ----------------------

#[derive(Clone)]
pub struct AppState {
    pub db: Db,
    pub user_service: UserService,
    pub auth_service: AuthService,
    pub group_service: GroupService,
}

pub type SharedState = Arc<AppState>;
