use crate::services::auth::AuthService;
use crate::services::group::GroupService;
use crate::services::proposal::ProposalService;
use crate::services::transaction::TransactionService;
use crate::services::user::UserService;
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
    pub transaction_service: TransactionService,
}

pub type SharedState = Arc<AppState>;
