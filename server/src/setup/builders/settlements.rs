use std::sync::Arc;

use crate::application::balances::BalancesService;
use crate::application::group::traits::repository::GroupRepository;
use crate::application::settlements::get_settlements::GetSettlementsUseCase;
use crate::application::settlements::pay_settlement::PaySettlementUseCase;
use crate::application::settlements::service::SettlementsService;
use crate::application::treasury::traits::{
    group_wallet_repo::GroupWalletRepository, transaction_repo::TransactionRepository,
    user_wallet_repo::UserWalletRepository,
};

pub fn build_settlements_service(
    balances_service: BalancesService,
    group_repo: Arc<dyn GroupRepository>,
    user_wallet_repo: Arc<dyn UserWalletRepository>,
    group_wallet_repo: Arc<dyn GroupWalletRepository>,
    transaction_repo: Arc<dyn TransactionRepository>,
) -> SettlementsService {
    SettlementsService {
        get_settlements: GetSettlementsUseCase { balances_service },
        pay_settlement: PaySettlementUseCase {
            group_repo,
            user_wallet_repo,
            group_wallet_repo,
            transaction_repo,
        },
    }
}
