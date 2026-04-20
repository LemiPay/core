#[cfg(test)]
mod tests {
    use uuid::Uuid;

    use crate::core::core::core;
    use crate::core::tests::helpers::*;

    /// El pagador queda con saldo positivo, los demás negativos.
    #[test]
    fn expense_payer_gets_credited_others_debited() {
        let alice = Uuid::new_v4();
        let bob = Uuid::new_v4();

        // Expense de 90: alice paga por todos
        // add alice 90 → alice=90 bob=0  grupo=90
        // remove_all 90 → alice=45 bob=-45 grupo=0
        let result = core(
            vec![alice, bob],
            vec![],
            vec![make_expense(alice, "90")],
        );

        assert_eq!(result.get_user_balance(&alice).unwrap(), &dec("45"));
        assert_eq!(result.get_user_balance(&bob).unwrap(), &dec("-45"));
        assert_eq!(result.get_group_balance(), &dec("0"));
    }

    /// La suma de todos los balances individuales siempre iguala al group_balance.
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

        let result = core(users, txs, expenses);

        let sum: bigdecimal::BigDecimal = result.get_all_balances().values().cloned().sum();
        assert_eq!(&sum, result.get_group_balance());
    }

    /// Expense de usuario que no está en el mapa → panic de integridad.
    #[test]
    #[should_panic(expected = "Error de Integridad: Usuario de expense no encontrado.")]
    fn expense_unknown_user_panics() {
        let valid = Uuid::new_v4();
        let ghost = Uuid::new_v4();
        core(vec![valid], vec![], vec![make_expense(ghost, "50")]);
    }
}