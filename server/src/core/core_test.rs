#[cfg(test)]
mod tests {
    use crate::core::core::core;
    use crate::models::transaction::{MyTransactionType, Transaction};
    use bigdecimal::{BigDecimal, Zero};
    use chrono::NaiveDateTime;
    use std::str::FromStr;
    use uuid::Uuid;
    // ── helpers ──────────────────────────────────────────────────────────────

    fn dec(s: &str) -> BigDecimal {
        BigDecimal::from_str(s).unwrap()
    }

    fn dummy_dt() -> NaiveDateTime {
        NaiveDateTime::from_timestamp_opt(0, 0).unwrap()
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

    // ── tests ─────────────────────────────────────────────────────────────────

    /// Un usuario deposita → su balance sube, el grupo sube igual.
    #[test]
    fn test_single_deposit() {
        let user = Uuid::new_v4();
        let txs = vec![make_tx(user, "100.00", MyTransactionType::Deposit)];

        let result = core(vec![user], txs);

        assert_eq!(result.get_user_balance(&user).unwrap(), &dec("100.00"));
        assert_eq!(result.get_group_balance(), &dec("100.00"));
    }

    /// Varios usuarios depositan distintos montos → cada balance es independiente.
    #[test]
    fn test_multiple_deposits_independent_balances() {
        let alice = Uuid::new_v4();
        let bob = Uuid::new_v4();

        let txs = vec![
            make_tx(alice, "200.00", MyTransactionType::Deposit),
            make_tx(bob, "50.00", MyTransactionType::Deposit),
            make_tx(alice, "30.00", MyTransactionType::Deposit),
        ];

        let result = core(vec![alice, bob], txs);

        assert_eq!(result.get_user_balance(&alice).unwrap(), &dec("230.00"));
        assert_eq!(result.get_user_balance(&bob).unwrap(), &dec("50.00"));
        assert_eq!(result.get_group_balance(), &dec("280.00"));
    }

    /// Withdraw se reparte equitativamente entre todos los miembros.
    #[test]
    fn test_withdraw_splits_evenly() {
        let alice = Uuid::new_v4();
        let bob = Uuid::new_v4();
        let carol = Uuid::new_v4();

        // Primero depositamos para tener fondos
        let mut txs = vec![
            make_tx(alice, "300.00", MyTransactionType::Deposit),
            make_tx(bob, "300.00", MyTransactionType::Deposit),
            make_tx(carol, "300.00", MyTransactionType::Deposit),
        ];
        // Gastamos 150 del grupo → 50 por cabeza
        txs.push(make_tx(
            Uuid::new_v4(),
            "150.00",
            MyTransactionType::Withdraw,
        ));

        let result = core(vec![alice, bob, carol], txs);

        assert_eq!(result.get_user_balance(&alice).unwrap(), &dec("250.00"));
        assert_eq!(result.get_user_balance(&bob).unwrap(), &dec("250.00"));
        assert_eq!(result.get_user_balance(&carol).unwrap(), &dec("250.00"));
        assert_eq!(result.get_group_balance(), &dec("750.00"));
    }

    /// Flujo completo: depósito → gasto → depósito → gasto.
    #[test]
    fn test_deposit_then_withdraw_flow() {
        let alice = Uuid::new_v4();
        let bob = Uuid::new_v4();

        let txs = vec![
            make_tx(alice, "100.00", MyTransactionType::Deposit),
            make_tx(bob, "100.00", MyTransactionType::Deposit),
            // Gastan 40 entre los dos → -20 c/u
            make_tx(Uuid::new_v4(), "40.00", MyTransactionType::Withdraw),
            make_tx(alice, "10.00", MyTransactionType::Deposit),
            // Gastan otros 20 → -10 c/u
            make_tx(Uuid::new_v4(), "20.00", MyTransactionType::Withdraw),
        ];

        let result = core(vec![alice, bob], txs);

        // alice: 100 + 10 - 20 - 10 = 80
        // bob:   100       - 20 - 10 = 70
        assert_eq!(result.get_user_balance(&alice).unwrap(), &dec("80.00"));
        assert_eq!(result.get_user_balance(&bob).unwrap(), &dec("70.00"));
        assert_eq!(result.get_group_balance(), &dec("150.00"));
    }

    /// Sin transacciones → todo queda en cero.
    #[test]
    fn test_no_transactions_all_zero() {
        let user = Uuid::new_v4();

        let result = core(vec![user], vec![]);

        assert_eq!(result.get_user_balance(&user).unwrap(), &BigDecimal::zero());
        assert_eq!(result.get_group_balance(), &BigDecimal::zero());
    }

    /// Deposit de un usuario que no está en el grupo → panic con el mensaje de integridad.
    #[test]
    #[should_panic(expected = "Error de Integridad: Usuario no encontrado.")]
    fn test_deposit_unknown_user_panics() {
        let valid_user = Uuid::new_v4();
        let ghost_user = Uuid::new_v4(); // no está en users_id

        let txs = vec![make_tx(ghost_user, "50.00", MyTransactionType::Deposit)];

        // Solo pasamos valid_user → ghost_user no existe en el mapa
        core(vec![valid_user], txs);
    }

    /// Withdraw mayor al balance del grupo → panic por fondos insuficientes.
    #[test]
    #[should_panic(expected = "Error de Integridad: Fondos insuficientes.")]
    fn test_withdraw_exceeds_group_balance_panics() {
        let alice = Uuid::new_v4();

        let txs = vec![
            make_tx(alice, "100.00", MyTransactionType::Deposit),
            make_tx(Uuid::new_v4(), "999.00", MyTransactionType::Withdraw),
        ];

        core(vec![alice], txs);
    }
}
