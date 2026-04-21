#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::core::core::{CoreError, core};
    use crate::core::tests::helpers::*;

    #[test]
    fn expense_payer_gets_credited_others_debited() {
        let alice = Uuid::new_v4();
        let bob = Uuid::new_v4();

        let result = core(vec![alice, bob], vec![], vec![make_expense(alice, "90")]).unwrap();

        assert_eq!(result.get_user_balance(&alice).unwrap(), &dec("45"));
        assert_eq!(result.get_user_balance(&bob).unwrap(), &dec("-45"));
        assert_eq!(result.get_group_balance(), &dec("0"));
    }

    #[test]
    fn balances_sum_equals_group_balance() {
        let alice = Uuid::new_v4();
        let bob = Uuid::new_v4();
        let carol = Uuid::new_v4();
        let users = vec![alice, bob, carol];

        let txs = vec![
            make_deposit(alice, "100"),
            make_deposit(bob, "100"),
            make_deposit(carol, "100"),
        ];
        let expenses = vec![make_expense(alice, "60")];

        let result = core(users, txs, expenses).unwrap();

        let sum: bigdecimal::BigDecimal = result.get_all_balances().values().cloned().sum();
        assert_eq!(&sum, result.get_group_balance());
    }

    #[test]
    fn expense_unknown_user_returns_error() {
        let valid = Uuid::new_v4();
        let ghost = Uuid::new_v4();
        let result = core(vec![valid], vec![], vec![make_expense(ghost, "50")]);

        assert!(matches!(result, Err(CoreError::UserNotFound)));
    }
}
