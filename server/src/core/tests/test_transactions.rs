#[cfg(test)]
mod tests {
    use bigdecimal::Zero;
    use uuid::Uuid;

    use crate::core::core::{core, create_empty_map};
    use crate::core::tests::helpers::*;

    #[test]
    fn deposit_increases_user_and_group_balance() {
        let user = Uuid::new_v4();
        let result = core(vec![user], vec![make_deposit(user, "100")], vec![]);

        assert_eq!(result.get_user_balance(&user).unwrap(), &dec("100"));
        assert_eq!(result.get_group_balance(), &dec("100"));
    }

    #[test]
    fn multiple_deposits_are_independent() {
        let alice = Uuid::new_v4();
        let bob = Uuid::new_v4();

        let txs = vec![
            make_deposit(alice, "200"),
            make_deposit(bob, "50"),
            make_deposit(alice, "30"),
        ];
        let result = core(vec![alice, bob], txs, vec![]);

        assert_eq!(result.get_user_balance(&alice).unwrap(), &dec("230"));
        assert_eq!(result.get_user_balance(&bob).unwrap(), &dec("50"));
        assert_eq!(result.get_group_balance(), &dec("280"));
    }

    #[test]
    fn withdraw_splits_evenly() {
        let alice = Uuid::new_v4();
        let bob = Uuid::new_v4();
        let carol = Uuid::new_v4();
        let users = vec![alice, bob, carol];

        let txs = vec![
            make_deposit(alice, "300"),
            make_deposit(bob, "300"),
            make_deposit(carol, "300"),
            make_withdraw("150"),
        ];
        let result = core(users, txs, vec![]);

        assert_eq!(result.get_user_balance(&alice).unwrap(), &dec("250"));
        assert_eq!(result.get_user_balance(&bob).unwrap(), &dec("250"));
        assert_eq!(result.get_user_balance(&carol).unwrap(), &dec("250"));
        assert_eq!(result.get_group_balance(), &dec("750"));
    }

    #[test]
    fn no_transactions_all_zero() {
        let user = Uuid::new_v4();
        let result = core(vec![user], vec![], vec![]);

        assert_eq!(result.get_user_balance(&user).unwrap(), &dec("0"));
        assert_eq!(result.get_group_balance(), &dec("0"));
    }

    #[test]
    #[should_panic(expected = "Error de Integridad: Usuario no encontrado.")]
    fn deposit_unknown_user_panics() {
        let valid = Uuid::new_v4();
        let ghost = Uuid::new_v4();
        core(vec![valid], vec![make_deposit(ghost, "50")], vec![]);
    }

    #[test]
    #[should_panic(expected = "Error de Integridad: Fondos insuficientes.")]
    fn withdraw_exceeds_balance_panics() {
        let alice = Uuid::new_v4();
        let txs = vec![make_deposit(alice, "100"), make_withdraw("999")];
        core(vec![alice], txs, vec![]);
    }
}
