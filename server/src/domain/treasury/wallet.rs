use crate::domain::treasury::money::Money;

pub struct Wallet {
    pub id: WalletId,
    pub owner: WalletOwner, // User | Group
    pub balance: Money,
}