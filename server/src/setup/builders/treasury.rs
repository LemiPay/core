use std::sync::Arc;

use crate::application::treasury::list_user_transactions::ListUserTransactionsUseCase;
use crate::application::treasury::{
    TreasuryService, create_group_wallet::CreateGroupWalletUseCase,
    create_user_wallet::CreateUserWalletUseCase, fund_group::FundGroupUseCase,
    fund_wallet::FundWalletUseCase, get_group_transaction::GetGroupTransactionUseCase,
    get_user_wallet_by_address_and_ticker::GetUserWalletByAddressAndTickerUseCase,
    list_group_transactions::ListGroupTransactionsUseCase,
    list_group_wallets::ListGroupWalletsUseCase, list_user_wallets::ListUserWalletsUseCase,
    transfer_funds::TransferFundsUseCase, withdraw_wallet::WithdrawWalletUseCase,
};
use crate::infrastructure::db::repositories::{
    currency_repo_impl::DieselCurrencyRepository,
    group_wallet_repo_impl::DieselGroupWalletRepository,
    transaction_repo_impl::DieselTransactionRepository,
    user_wallet_repo_impl::DieselUserWalletRepository,
};

pub fn build_treasury_service(
    user_wallet_repo: Arc<DieselUserWalletRepository>,
    group_wallet_repo: Arc<DieselGroupWalletRepository>,
    transaction_repo: Arc<DieselTransactionRepository>,
    currency_repo: Arc<DieselCurrencyRepository>,
) -> TreasuryService {
    TreasuryService {
        // User wallet
        create_user_wallet: CreateUserWalletUseCase {
            user_wallet_repo: user_wallet_repo.clone(),
            currency_repo: currency_repo.clone(),
        },
        fund_wallet: FundWalletUseCase {
            user_wallet_repo: user_wallet_repo.clone(),
        },
        withdraw_wallet: WithdrawWalletUseCase {
            user_wallet_repo: user_wallet_repo.clone(),
        },
        transfer_funds: TransferFundsUseCase {
            user_wallet_repo: user_wallet_repo.clone(),
        },
        list_user_wallets: ListUserWalletsUseCase {
            user_wallet_repo: user_wallet_repo.clone(),
        },
        get_user_wallet_by_address_and_ticker: GetUserWalletByAddressAndTickerUseCase {
            user_wallet_repo: user_wallet_repo.clone(),
            currency_repo: currency_repo.clone(),
        },

        // Group wallet
        create_group_wallet: CreateGroupWalletUseCase {
            group_wallet_repo: group_wallet_repo.clone(),
            currency_repo,
        },
        list_group_wallets: ListGroupWalletsUseCase {
            group_wallet_repo: group_wallet_repo.clone(),
        },

        // Transactions
        fund_group: FundGroupUseCase {
            user_wallet_repo,
            group_wallet_repo,
            transaction_repo: transaction_repo.clone(),
        },
        list_group_transactions: ListGroupTransactionsUseCase {
            transaction_repo: transaction_repo.clone(),
        },
        list_user_transactions: ListUserTransactionsUseCase {
            transaction_repo: transaction_repo.clone(),
        },
        get_group_transaction: GetGroupTransactionUseCase { transaction_repo },
    }
}
