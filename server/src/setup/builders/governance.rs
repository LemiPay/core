use std::sync::Arc;

use crate::{
    application::governance::GovernanceService,
    infrastructure::db::repositories::{
        governance_repo_impl::DieselGovernanceRepository, group_repo_impl::DieselGroupRepository,
        user_repo_impl::DieselUserRepository, user_wallet_repo_impl::DieselUserWalletRepository,
    },
};

pub fn build_governance_service(
    governance_repo: Arc<DieselGovernanceRepository>,
    group_repo: Arc<DieselGroupRepository>,
    user_repo: Arc<DieselUserRepository>,
    user_wallet_repo: Arc<DieselUserWalletRepository>,
) -> GovernanceService {
    GovernanceService {
        governance_repo,
        group_repo,
        user_repo,
        user_wallet_repo,
    }
}
