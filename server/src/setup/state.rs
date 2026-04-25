use std::sync::Arc;

use crate::application::auth::{login::LoginUseCase, me::GetMeUseCase, register::RegisterUseCase};

use super::config::AppConfig;

// ----------------------
// APP STATE
// ----------------------

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,

    pub get_me_use_case: Arc<GetMeUseCase>,
    pub login_use_case: Arc<LoginUseCase>,
    pub register_use_case: Arc<RegisterUseCase>,
}
