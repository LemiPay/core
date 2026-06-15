use crate::infrastructure::blockchain::contracts::lemipay_vault::LemiPayVault::{
    FeeUpdated, FeeWalletUpdated, Fund, OwnershipTransferred, Paused, TokenAdded, TokenDisabled,
    Unpaused, Withdraw,
};
use crate::infrastructure::blockchain::{
    ContractEvent, FeeUpdatedData, FeeWalletUpdatedData, FundData, OwnershipTransferredData,
    PausedData, TokenAddedData, TokenDisabledData, UnpausedData, WithdrawData,
};
use alloy::rpc::types::Log;

pub fn try_decode_event(log: Log) -> Option<ContractEvent> {
    type Decoder = fn(&Log) -> Option<ContractEvent>;

    static DECODERS: &[Decoder] = &[
        |log| {
            log.log_decode::<TokenAdded>().ok().map(|d| {
                let e = d.data();
                ContractEvent::TokenAdded(TokenAddedData {
                    token: e.token,
                    currency_id: e.currencyId,
                })
            })
        },
        |log| {
            log.log_decode::<Fund>().ok().map(|d| {
                let e = d.data();
                ContractEvent::Fund(FundData {
                    sender: e.sender,
                    wallet_address: e.walletAddress,
                    token: e.token,
                    currency_id: e.currencyId,
                    gross_amount: e.grossAmount,
                    fee_amount: e.feeAmount,
                    net_amount: e.netAmount,
                    tx_hash: log.transaction_hash.unwrap_or_default(),
                    block_number: log.block_number.unwrap_or(0),
                })
            })
        },
        |log| {
            log.log_decode::<Withdraw>().ok().map(|d| {
                let e = d.data();
                ContractEvent::Withdraw(WithdrawData {
                    receiver: e.receiver,
                    wallet_address: e.walletAddress,
                    token: e.token,
                    currency_id: e.currencyId,
                    gross_amount: e.grossAmount,
                    fee_amount: e.feeAmount,
                    net_amount: e.netAmount,
                    tx_hash: log.transaction_hash.unwrap_or_default(),
                    block_number: log.block_number.unwrap_or(0),
                })
            })
        },
        |log| {
            log.log_decode::<FeeUpdated>().ok().map(|d| {
                let e = d.data();
                ContractEvent::FeeUpdated(FeeUpdatedData {
                    old_fee: e.oldFeeBps,
                    new_fee: e.newFeeBps,
                })
            })
        },
        |log| {
            log.log_decode::<FeeWalletUpdated>().ok().map(|d| {
                let e = d.data();
                ContractEvent::FeeWalletUpdated(FeeWalletUpdatedData {
                    old_fee_wallet: e.oldWallet,
                    new_fee_wallet: e.newWallet,
                })
            })
        },
        |log| {
            log.log_decode::<OwnershipTransferred>().ok().map(|d| {
                let e = d.data();
                ContractEvent::OwnershipTransferred(OwnershipTransferredData {
                    previous_owner: e.previousOwner,
                    new_owner: e.newOwner,
                })
            })
        },
        |log| {
            log.log_decode::<Paused>().ok().map(|d| {
                let e = d.data();
                ContractEvent::Paused(PausedData { account: e.account })
            })
        },
        |log| {
            log.log_decode::<Unpaused>().ok().map(|d| {
                let e = d.data();
                ContractEvent::Unpaused(UnpausedData { account: e.account })
            })
        },
        |log| {
            log.log_decode::<TokenDisabled>().ok().map(|d| {
                let e = d.data();
                ContractEvent::TokenDisabled(TokenDisabledData { token: e.token })
            })
        },
    ];

    DECODERS.iter().find_map(|decode| decode(&log))
}
