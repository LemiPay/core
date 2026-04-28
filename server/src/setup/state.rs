use std::sync::Arc;

use crate::application::{
    auth::{login::LoginUseCase, register::RegisterUseCase},
    users::{get_user::UserUseCase, me::GetMeUseCase},
};

use super::config::AppConfig;

// ----------------------
// APP STATE
// ----------------------

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,

    // Auth
    pub get_me_use_case: Arc<GetMeUseCase>,
    pub login_use_case: Arc<LoginUseCase>,
    pub register_use_case: Arc<RegisterUseCase>,

    // Users
    pub user_use_case: Arc<UserUseCase>,
}
