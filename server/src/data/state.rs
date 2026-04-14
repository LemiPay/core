use crate::services::auth::AuthService;
use crate::services::group::GroupService;
use crate::services::group_wallet::GroupWalletService;
use crate::services::proposal::ProposalService;
use crate::services::user::UserService;
use crate::services::user_wallet::UserWalletService;
use std::sync::Arc;
// ----------------------
// APP STATE
// ----------------------

#[derive(Clone)]
pub struct AppState {
    pub user_service: UserService,
    pub auth_service: AuthService,
    pub group_service: GroupService,
    pub proposal_service: ProposalService,
    pub user_wallet_service: Arc<UserWalletService>,
    pub group_wallet_service: GroupWalletService,
}

pub type SharedState = Arc<AppState>;
