use crate::domain::{
    treasury::{
        error::TreasuryError, group_wallet::GroupWallet, money::Money, user_wallet::UserWallet,
    },
    user::UserId,
};

pub struct TreasuryPolicy;

impl TreasuryPolicy {
    pub fn ensure_owns_wallet(wallet: &UserWallet, user_id: UserId) -> Result<(), TreasuryError> {
        if wallet.is_owned_by(user_id) {
            Ok(())
        } else {
            Err(TreasuryError::InvalidAddress)
        }
    }

    pub fn ensure_distinct_addresses(
        sender: &UserWallet,
        receiver_address: &str,
    ) -> Result<(), TreasuryError> {
        if sender.address == receiver_address {
            Err(TreasuryError::SameWalletTransfer)
        } else {
            Ok(())
        }
    }

    pub fn ensure_user_can_cover(wallet: &UserWallet, amount: &Money) -> Result<(), TreasuryError> {
        if wallet.balance.has_enough(amount)? {
            Ok(())
        } else {
            Err(TreasuryError::InsufficientFunds)
        }
    }

    pub fn ensure_group_can_cover(
        wallet: &GroupWallet,
        amount: &Money,
    ) -> Result<(), TreasuryError> {
        if wallet.balance.has_enough(amount)? {
            Ok(())
        } else {
            Err(TreasuryError::InsufficientFunds)
        }
    }
}
