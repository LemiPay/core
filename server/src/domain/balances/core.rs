use crate::domain::{
    balances::{BalancesError, BalancesMap},
    expense::Expense,
    treasury::{Transaction, TransactionType},
    user::UserId,
};

/// Legacy CORE logic to reconstruct group balances from historical events.
///
/// This is intentionally pure and deterministic.
pub fn core(
    users_id: Vec<UserId>,
    transactions: Vec<Transaction>,
    expenses: Vec<Expense>,
) -> Result<BalancesMap, BalancesError> {
    let balances = BalancesMap::empty_for(&users_id);
    let balances = read_all_tx(transactions, balances)?;
    read_all_expenses(expenses, balances)
}

fn read_all_tx(
    transactions: Vec<Transaction>,
    mut balances: BalancesMap,
) -> Result<BalancesMap, BalancesError> {
    for tx in transactions {
        balances = match tx.tx_type {
            TransactionType::Deposit => {
                balances.add_balance_to_user(tx.user_id, tx.amount.amount.clone())?
            }
            TransactionType::Withdraw => balances.remove_to_all(tx.amount.amount)?,
            _ => balances,
        };
    }
    Ok(balances)
}

fn read_all_expenses(
    expenses: Vec<Expense>,
    mut balances: BalancesMap,
) -> Result<BalancesMap, BalancesError> {
    for expense in expenses {
        balances = balances.add_balance_to_user(expense.user_id, expense.amount.clone())?;
        balances = balances.remove_to_all(expense.amount)?;
    }
    Ok(balances)
}

#[cfg(test)]
mod tests {
    use bigdecimal::BigDecimal;
    use chrono::Utc;
    use std::str::FromStr;
    use uuid::Uuid;

    use crate::domain::{
        expense::{Expense, ExpenseId, ExpenseStatus},
        group::GroupId,
        treasury::{CurrencyId, Money, Transaction, TransactionId, TransactionType},
        user::UserId,
    };

    use super::*;

    fn dec(value: &str) -> BigDecimal {
        BigDecimal::from_str(value).expect("valid decimal")
    }

    fn make_deposit(user_id: UserId, amount: &str) -> Transaction {
        Transaction {
            id: TransactionId(Uuid::new_v4()),
            tx_hash: None,
            amount: Money {
                amount: dec(amount),
                currency: CurrencyId(Uuid::new_v4()),
            },
            user_id,
            group_id: GroupId(Uuid::new_v4()),
            address: "addr".to_string(),
            description: None,
            tx_type: TransactionType::Deposit,
        }
    }

    fn make_withdraw(amount: &str) -> Transaction {
        Transaction {
            id: TransactionId(Uuid::new_v4()),
            tx_hash: None,
            amount: Money {
                amount: dec(amount),
                currency: CurrencyId(Uuid::new_v4()),
            },
            user_id: UserId(Uuid::new_v4()),
            group_id: GroupId(Uuid::new_v4()),
            address: "addr".to_string(),
            description: None,
            tx_type: TransactionType::Withdraw,
        }
    }

    fn make_expense(user_id: UserId, amount: &str) -> Expense {
        let now = Utc::now().naive_utc();
        Expense::rehydrate(
            ExpenseId(Uuid::new_v4()),
            user_id,
            GroupId(Uuid::new_v4()),
            CurrencyId(Uuid::new_v4()),
            None,
            dec(amount),
            ExpenseStatus::Created,
            now,
            now,
        )
    }

    #[test]
    fn deposit_increases_user_and_group_balance() {
        let user = UserId(Uuid::new_v4());
        let result = core(vec![user], vec![make_deposit(user, "100")], vec![]).unwrap();

        assert_eq!(result.get_user_balance(&user).unwrap(), &dec("100"));
        assert_eq!(result.get_group_balance(), &dec("100"));
    }

    #[test]
    fn withdraw_exceeds_balance_returns_error() {
        let user = UserId(Uuid::new_v4());
        let txs = vec![make_deposit(user, "100"), make_withdraw("999")];
        let result = core(vec![user], txs, vec![]);
        assert!(matches!(result, Err(BalancesError::InsufficientFunds)));
    }

    #[test]
    fn expense_unknown_user_returns_error() {
        let valid = UserId(Uuid::new_v4());
        let ghost = UserId(Uuid::new_v4());
        let result = core(vec![valid], vec![], vec![make_expense(ghost, "50")]);
        assert!(matches!(result, Err(BalancesError::UserNotFound)));
    }

    #[test]
    fn integration_case_four_members() {
        let facu = UserId(Uuid::new_v4());
        let mate = UserId(Uuid::new_v4());
        let pepe = UserId(Uuid::new_v4());
        let juan = UserId(Uuid::new_v4());
        let users = vec![facu, mate, pepe, juan];

        let cuota = "13.333333333333333333";
        let txs = vec![
            make_deposit(facu, "20"),
            make_deposit(mate, cuota),
            make_deposit(pepe, cuota),
            make_deposit(juan, cuota),
            make_withdraw("40"),
        ];
        let expenses = vec![make_expense(facu, "100")];

        let result = core(users, txs, expenses).unwrap();

        assert_eq!(result.get_user_balance(&facu).unwrap(), &dec("85"));
        let expected_rest = dec(cuota) - dec("35");
        for user in [mate, pepe, juan] {
            assert_eq!(result.get_user_balance(&user).unwrap(), &expected_rest);
        }

        let sum: BigDecimal = result.get_all_balances().values().cloned().sum();
        assert_eq!(&sum, result.get_group_balance());
    }
}
