use std::fmt::{Display, Formatter};
use uuid::Uuid;

use crate::domain::{
    group::GroupId,
    treasury::{currency::CurrencyId, money::Money},
    user::UserId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TransactionId(pub Uuid);

impl TransactionId {
    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Display for TransactionId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransactionType {
    Deposit,
    Withdraw,
    Expense,
    Investment,
}

#[derive(Debug, Clone)]
pub struct Transaction {
    pub id: TransactionId,
    pub tx_hash: Option<String>,
    pub amount: Money,
    pub user_id: UserId,
    pub group_id: GroupId,
    pub address: String,
    pub description: Option<String>,
    pub tx_type: TransactionType,
}

#[derive(Debug, Clone)]
pub struct NewTransaction {
    pub tx_hash: Option<String>,
    pub amount: Money,
    pub user_id: UserId,
    pub group_id: GroupId,
    pub currency_id: CurrencyId,
    pub address: String,
    pub description: Option<String>,
    pub tx_type: TransactionType,
}
