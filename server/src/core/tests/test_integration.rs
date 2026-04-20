#[cfg(test)]
mod tests {
    use bigdecimal::BigDecimal;
    use std::str::FromStr;
    use uuid::Uuid;

    use crate::core::core::core;
    use crate::core::tests::helpers::*;

    /// Caso fundacional de LemiPay:
    ///
    /// Grupo: Facu, Mate, Pepe, Juan
    ///
    /// 1. Facu deposita 20
    /// 2. Mate, Pepe, Juan depositan 40/3 cada uno  → grupo = 60
    /// 3. Withdraw de 40                             → todos -10
    /// 4. Facu hace expense de 100
    ///
    /// Balances esperados:
    ///   facu = 85
    ///   mate = pepe = juan = 40/3 - 35 = -65/3 ≈ -21.666...
    ///   group_balance = 20  (lo que quedó del grupo tras el withdraw)
    #[test]
    fn caso_fundacional_4_integrantes() {
        let facu = Uuid::new_v4();
        let mate = Uuid::new_v4();
        let pepe = Uuid::new_v4();
        let juan = Uuid::new_v4();
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

        let result = core(users, txs, expenses);

        // ── Facu: 20 - 10 + 100 - 25 = 85 ──────────────────────────────────────
        assert_eq!(result.get_user_balance(&facu).unwrap(), &dec("85"));

        // ── Mate/Pepe/Juan: cuota - 10 - 25 = cuota - 35 ───────────────────────
        let esperado_resto = dec(cuota) - dec("35");
        for user in [mate, pepe, juan] {
            assert_eq!(result.get_user_balance(&user).unwrap(), &esperado_resto);
        }

        // ── Invariante: suma de balances individuales == group_balance ───────────
        // No hardcodeamos el group_balance porque acumula el error de precisión
        // de la cuota. Lo que importa es que el sistema sea internamente consistente.
        let sum: BigDecimal = result.get_all_balances().values().cloned().sum();
        assert_eq!(
            &sum,
            result.get_group_balance(),
            "La suma de balances individuales debe ser igual al group_balance"
        );
    }
}
