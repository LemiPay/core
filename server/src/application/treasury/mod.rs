pub mod dto;
pub mod traits;

// Use cases
pub mod create_group_wallet;
pub mod create_user_wallet;
pub mod fund_event;
pub mod fund_group;
pub mod fund_wallet;
pub mod get_group_transaction;
pub mod get_user_wallet_by_address_and_ticker;
pub mod list_group_transactions;
pub mod list_group_wallets;
pub mod list_user_transactions;
pub mod list_user_wallets;
pub mod transfer_funds;
pub mod withdraw_wallet;

pub mod service;
pub use service::TreasuryService;
