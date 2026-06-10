use alloy::primitives::{Address, B256, U256};

pub trait PrintCommand {
    fn print(&self);
}

#[derive(Debug, Clone)]
pub enum ContractEvent {
    TokenAdded(TokenAddedData),
    Fund(FundData),
    Withdraw(WithdrawData),
    FeeUpdated(FeeUpdatedData),
    FeeWalletUpdated(FeeWalletUpdatedData),
    OwnershipTransferred(OwnershipTransferredData),
    Paused(PausedData),
    Unpaused(UnpausedData),
    TokenDisabled(TokenDisabledData),
}

impl ContractEvent {
    pub fn execute_print(&self) {
        match self {
            ContractEvent::TokenAdded(data) => data.print(),
            ContractEvent::Fund(data) => data.print(),
            ContractEvent::Withdraw(data) => data.print(),
            ContractEvent::FeeUpdated(data) => data.print(),
            ContractEvent::FeeWalletUpdated(data) => data.print(),
            ContractEvent::OwnershipTransferred(data) => data.print(),
            ContractEvent::Paused(data) => data.print(),
            ContractEvent::Unpaused(data) => data.print(),
            ContractEvent::TokenDisabled(data) => data.print(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TokenAddedData {
    pub token: Address,
    pub currency_id: B256,
}

impl PrintCommand for TokenAddedData {
    fn print(&self) {
        println!(
            "      TokenAdded {{ token: {:?}, currency_id: {:?} }}",
            self.token, self.currency_id
        );
    }
}

#[derive(Debug, Clone)]
pub struct FundData {
    pub sender: Address,
    pub wallet_address: B256,
    pub token: Address,
    pub currency_id: B256,
    pub gross_amount: U256,
    pub fee_amount: U256,
    pub net_amount: U256,
}

impl PrintCommand for FundData {
    fn print(&self) {
        println!(
            "      Fund {{ sender: {:?}, wallet: {:?}, token: {:?}, gross: {} }}",
            self.sender, self.wallet_address, self.token, self.gross_amount
        );
    }
}

#[derive(Debug, Clone)]
pub struct WithdrawData {
    pub receiver: Address,
    pub wallet_address: B256,
    pub token: Address,
    pub currency_id: B256,
    pub gross_amount: U256,
    pub fee_amount: U256,
    pub net_amount: U256,
}

impl PrintCommand for WithdrawData {
    fn print(&self) {
        println!(
            "      Withdraw {{ receiver: {:?}, wallet: {:?}, token: {:?}, gross: {} }}",
            self.receiver, self.wallet_address, self.token, self.gross_amount
        );
    }
}

#[derive(Debug, Clone)]
pub struct FeeUpdatedData {
    pub old_fee: U256,
    pub new_fee: U256,
}

impl PrintCommand for FeeUpdatedData {
    fn print(&self) {
        println!(
            "      FeeUpdated {{ old: {}, new: {} }}",
            self.old_fee, self.new_fee
        );
    }
}

#[derive(Debug, Clone)]
pub struct FeeWalletUpdatedData {
    pub old_fee_wallet: Address,
    pub new_fee_wallet: Address,
}

impl PrintCommand for FeeWalletUpdatedData {
    fn print(&self) {
        println!(
            "      FeeWalletUpdated {{ old: {:?}, new: {:?} }}",
            self.old_fee_wallet, self.new_fee_wallet
        );
    }
}

#[derive(Debug, Clone)]
pub struct OwnershipTransferredData {
    pub previous_owner: Address,
    pub new_owner: Address,
}

impl PrintCommand for OwnershipTransferredData {
    fn print(&self) {
        println!(
            "      OwnershipTransferred {{ from: {:?}, to: {:?} }}",
            self.previous_owner, self.new_owner
        );
    }
}

#[derive(Debug, Clone)]
pub struct PausedData {
    pub account: Address,
}

impl PrintCommand for PausedData {
    fn print(&self) {
        println!("      Paused {{ account: {:?} }}", self.account);
    }
}

#[derive(Debug, Clone)]
pub struct UnpausedData {
    pub account: Address,
}

impl PrintCommand for UnpausedData {
    fn print(&self) {
        println!("      Unpaused {{ account: {:?} }}", self.account);
    }
}

#[derive(Debug, Clone)]
pub struct TokenDisabledData {
    pub token: Address,
}

impl PrintCommand for TokenDisabledData {
    fn print(&self) {
        println!("      TokenDisabled {{ token: {:?} }}", self.token);
    }
}
