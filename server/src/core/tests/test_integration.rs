#[cfg(test)]
mod tests {
    use bigdecimal::BigDecimal;
    use uuid::Uuid;

    use crate::core::core::core;
    use crate::core::tests::helpers::*;

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

        let result = core(users, txs, expenses).unwrap();

        assert_eq!(result.get_user_balance(&facu).unwrap(), &dec("85"));

        let esperado_resto = dec(cuota) - dec("35");
        for user in [mate, pepe, juan] {
            assert_eq!(result.get_user_balance(&user).unwrap(), &esperado_resto);
        }

        let sum: BigDecimal = result.get_all_balances().values().cloned().sum();
        assert_eq!(
            &sum,
            result.get_group_balance(),
            "La suma de balances individuales debe ser igual al group_balance"
        );
    }
}
