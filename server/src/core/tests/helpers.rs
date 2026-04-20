use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use std::str::FromStr;
use uuid::Uuid;

use crate::models::expense::{Expense, MyExpenseStatus};
use crate::models::transaction::{MyTransactionType, Transaction};

pub fn dec(s: &str) -> BigDecimal {
    BigDecimal::from_str(s).unwrap()
}

pub fn dummy_dt() -> NaiveDateTime {
    NaiveDateTime::from_timestamp_opt(0, 0).unwrap()
}

pub fn make_deposit(user_id: Uuid, amount: &str) -> Transaction {
    make_tx(user_id, amount, MyTransactionType::Deposit)
}

pub fn make_withdraw(amount: &str) -> Transaction {
    make_tx(Uuid::new_v4(), amount, MyTransactionType::Withdraw)
}

pub fn make_expense(user_id: Uuid, amount: &str) -> Expense {
    Expense {
        expense_id: Uuid::new_v4(),
        user_id,
        currency_id: Uuid::new_v4(),
        group_id: Uuid::new_v4(),
        description: None,
        amount: dec(amount),
        status: MyExpenseStatus::Verified,
        created_at: dummy_dt(),
        updated_at: dummy_dt(),
    }
}

fn make_tx(user_id: Uuid, amount: &str, tx_type: MyTransactionType) -> Transaction {
    Transaction {
        id: Uuid::new_v4(),
        tx_hash: None,
        amount: dec(amount),
        user_id,
        group_id: Uuid::new_v4(),
        currency_id: Uuid::new_v4(),
        address: "0x0".to_string(),
        description: None,
        tx_type,
        created_at: dummy_dt(),
        updated_at: dummy_dt(),
    }
}