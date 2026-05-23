use std::sync::Arc;

use crate::{
    application::investment::InvestmentService,
    infrastructure::db::repositories::{
        group_repo_impl::DieselGroupRepository,
        group_wallet_repo_impl::DieselGroupWalletRepository,
        investment_repo_impl::DieselInvestmentRepository,
    },
};

pub fn build_investment_service(
    investment_repo: Arc<DieselInvestmentRepository>,
    group_repo: Arc<DieselGroupRepository>,
    group_wallet_repo: Arc<DieselGroupWalletRepository>,
) -> InvestmentService {
    InvestmentService {
        investment_repo,
        group_repo,
        group_wallet_repo,
    }
}
