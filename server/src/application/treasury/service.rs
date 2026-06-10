use super::{
    create_group_wallet::CreateGroupWalletUseCase, create_user_wallet::CreateUserWalletUseCase,
    fund_group::FundGroupUseCase, fund_wallet::FundWalletUseCase,
    get_group_transaction::GetGroupTransactionUseCase,
    get_user_wallet_by_address_and_ticker::GetUserWalletByAddressAndTickerUseCase,
    list_group_transactions::ListGroupTransactionsUseCase,
    list_group_wallets::ListGroupWalletsUseCase, list_user_wallets::ListUserWalletsUseCase,
    transfer_funds::TransferFundsUseCase, withdraw_wallet::WithdrawWalletUseCase,
};
use crate::application::treasury::list_user_transactions::ListUserTransactionsUseCase;

pub struct TreasuryService {
    // User wallet
    pub create_user_wallet: CreateUserWalletUseCase,
    pub fund_wallet: FundWalletUseCase,
    pub withdraw_wallet: WithdrawWalletUseCase,
    pub transfer_funds: TransferFundsUseCase,
    pub list_user_wallets: ListUserWalletsUseCase,
    pub get_user_wallet_by_address_and_ticker: GetUserWalletByAddressAndTickerUseCase,

    // Group wallet
    pub create_group_wallet: CreateGroupWalletUseCase,
    pub list_group_wallets: ListGroupWalletsUseCase,

    // Transactions
    pub fund_group: FundGroupUseCase,
    pub list_group_transactions: ListGroupTransactionsUseCase,
    pub list_user_transactions: ListUserTransactionsUseCase,
    pub get_group_transaction: GetGroupTransactionUseCase,
}
