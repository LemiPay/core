use super::{
    create_group_wallet::CreateGroupWalletUseCase, create_user_wallet::CreateUserWalletUseCase,
    faucet_fund_wallet::FaucetFundWalletUseCase,
    faucet_withdraw_wallet::FaucetWithdrawWalletUseCase, fund_group::FundGroupUseCase,
    get_group_transaction::GetGroupTransactionUseCase,
    get_user_wallet_by_address_and_ticker::GetUserWalletByAddressAndTickerUseCase,
    list_group_transactions::ListGroupTransactionsUseCase,
    list_group_wallets::ListGroupWalletsUseCase, list_user_wallets::ListUserWalletsUseCase,
    transfer_funds::TransferFundsUseCase,
};
use crate::application::treasury::list_user_transactions::ListUserTransactionsUseCase;

pub struct TreasuryService {
    // User wallet
    pub create_user_wallet: CreateUserWalletUseCase,
    pub faucet_fund_wallet: FaucetFundWalletUseCase,
    pub faucet_withdraw_wallet: FaucetWithdrawWalletUseCase,
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
