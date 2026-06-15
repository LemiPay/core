use crate::domain::balances::BalancesMap;
use crate::domain::settlements::entity::Settlement;
use crate::domain::user::UserId;
use bigdecimal::{BigDecimal, Signed, Zero};

pub fn recommend_settlements(map: &BalancesMap) -> Vec<Settlement> {
    let mut creditors: Vec<(UserId, BigDecimal)> = map
        .get_all_balances()
        .iter()
        .filter(|(_, b)| b.is_positive())
        .map(|(u, b)| (u.clone(), b.clone()))
        .collect();
    creditors.sort_by(|a, b| b.1.cmp(&a.1));

    let mut debtors: Vec<(UserId, BigDecimal)> = map
        .get_all_balances()
        .iter()
        .filter(|(_, b)| b.is_negative())
        .map(|(u, b)| (u.clone(), b.abs()))
        .collect();
    debtors.sort_by(|a, b| b.1.cmp(&a.1));

    let mut result: Vec<Settlement> = Vec::new();
    let mut i: usize = 0;
    let mut j: usize = 0;
    while i < debtors.len() && j < creditors.len() {
        let amount = if debtors[i].1 < creditors[j].1 {
            debtors[i].1.clone()
        } else {
            creditors[j].1.clone()
        };
        result.push(Settlement {
            from: debtors[i].0.clone(),
            to: creditors[j].0.clone(),
            amount: amount.clone(),
        });
        debtors[i].1 -= &amount;
        creditors[j].1 -= &amount;
        if debtors[i].1.is_zero() {
            i += 1;
        }
        if creditors[j].1.is_zero() {
            j += 1;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use std::str::FromStr;
    use uuid::Uuid;

    fn bd(s: &str) -> BigDecimal {
        BigDecimal::from_str(s).unwrap()
    }

    fn user() -> UserId {
        UserId(Uuid::new_v4())
    }

    fn map_from(pairs: Vec<(UserId, &str)>) -> BalancesMap {
        let balances: HashMap<UserId, BigDecimal> =
            pairs.into_iter().map(|(u, b)| (u, bd(b))).collect();
        let group_balance = balances.values().fold(BigDecimal::zero(), |acc, b| acc + b);
        BalancesMap::new(balances, group_balance)
    }

    fn total_from(settlements: &[Settlement], u: UserId) -> BigDecimal {
        settlements
            .iter()
            .filter(|s| s.from == u)
            .fold(BigDecimal::zero(), |acc, s| acc + s.amount.clone())
    }

    /// Suma de todos los settlements hacia un usuario dado.
    fn total_to(settlements: &[Settlement], u: UserId) -> BigDecimal {
        settlements
            .iter()
            .filter(|s| s.to == u)
            .fold(BigDecimal::zero(), |acc, s| acc + s.amount.clone())
    }

    #[test]
    fn empty_map_returns_empty() {
        let map = map_from(vec![]);
        let result = recommend_settlements(&map);
        assert!(result.is_empty());
    }

    #[test]
    fn all_zero_balances_returns_empty() {
        let a = user();
        let b = user();
        let map = map_from(vec![(a, "0"), (b, "0")]);
        let result = recommend_settlements(&map);
        assert!(result.is_empty());
    }

    #[test]
    fn single_creditor_no_debtor_returns_empty() {
        let a = user();
        let map = map_from(vec![(a, "10")]);
        let result = recommend_settlements(&map);
        assert!(result.is_empty());
    }

    #[test]
    fn single_debtor_no_creditor_returns_empty() {
        let a = user();
        let map = map_from(vec![(a, "-10")]);
        let result = recommend_settlements(&map);
        assert!(result.is_empty());
    }

    #[test]
    fn simple_one_to_one() {
        let a = user(); // debe 10
        let b = user(); // le deben 10
        let map = map_from(vec![(a, "-10"), (b, "10")]);

        let result = recommend_settlements(&map);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].from, a);
        assert_eq!(result[0].to, b);
        assert_eq!(result[0].amount, bd("10"));
    }

    #[test]
    fn two_debtors_one_creditor_exact_split() {
        let a = user(); // -5
        let b = user(); // -5
        let c = user(); // +10
        let map = map_from(vec![(a, "-5"), (b, "-5"), (c, "10")]);

        let result = recommend_settlements(&map);

        assert_eq!(result.len(), 2);
        // todos los settlements van hacia c
        for s in &result {
            assert_eq!(s.to, c);
        }
        assert_eq!(total_to(&result, c), bd("10"));
        assert_eq!(total_from(&result, a), bd("5"));
        assert_eq!(total_from(&result, b), bd("5"));
    }

    #[test]
    fn one_debtor_two_creditors_exact_split() {
        let a = user(); // -10
        let b = user(); // +5
        let c = user(); // +5
        let map = map_from(vec![(a, "-10"), (b, "5"), (c, "5")]);

        let result = recommend_settlements(&map);

        assert_eq!(result.len(), 2);
        for s in &result {
            assert_eq!(s.from, a);
        }
        assert_eq!(total_from(&result, a), bd("10"));
        assert_eq!(total_to(&result, b), bd("5"));
        assert_eq!(total_to(&result, c), bd("5"));
    }

    #[test]
    fn mismatched_amounts_require_multiple_settlements() {
        // a debe 8, b debe 4 -> total deuda 12
        // c le deben 7, d le deben 5 -> total credito 12
        let a = user();
        let b = user();
        let c = user();
        let d = user();
        let map = map_from(vec![(a, "-8"), (b, "-4"), (c, "7"), (d, "5")]);

        let result = recommend_settlements(&map);

        // verificar consistencia global: lo que sale de deudores == lo que entra a acreedores
        assert_eq!(
            total_from(&result, a) + total_from(&result, b),
            total_to(&result, c) + total_to(&result, d)
        );

        // cada deudor paga exactamente su deuda
        assert_eq!(total_from(&result, a), bd("8"));
        assert_eq!(total_from(&result, b), bd("4"));

        // cada acreedor recibe exactamente lo que se le debe
        assert_eq!(total_to(&result, c), bd("7"));
        assert_eq!(total_to(&result, d), bd("5"));

        // no debe haber settlements de monto cero
        for s in &result {
            assert!(s.amount > BigDecimal::zero());
        }
    }

    #[test]
    fn high_precision_no_rounding_loss() {
        // valores con muchos decimales: si esto fuera f64 podria perder precision
        let a = user(); // debe 0.1 + 0.2 = 0.3 (clasico caso de error en floats)
        let b = user(); // le deben 0.3
        let map = map_from(vec![(a, "-0.3"), (b, "0.3")]);

        let result = recommend_settlements(&map);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].amount, bd("0.3"));
    }

    #[test]
    fn many_decimals_remainder_distribution() {
        // 100 dividido entre 3 no da exacto -> probamos que el algoritmo
        // sigue funcionando con decimales largos sin que nada quede colgado
        let a = user();
        let b = user();
        let c = user();
        let map = map_from(vec![
            (a, "-33.333333333333333333"),
            (b, "-33.333333333333333334"),
            (c, "66.666666666666666667"),
        ]);

        let result = recommend_settlements(&map);

        // suma total recibida por c == suma total pagada por a y b
        let total_in = total_to(&result, c);
        let total_out = total_from(&result, a) + total_from(&result, b);
        assert_eq!(total_in, total_out);

        // cada deudor paga exactamente su deuda
        assert_eq!(total_from(&result, a), bd("33.333333333333333333"));
        assert_eq!(total_from(&result, b), bd("33.333333333333333334"));
    }

    #[test]
    fn larger_random_like_scenario_balances_to_zero() {
        let a = user();
        let b = user();
        let c = user();
        let d = user();
        let e = user();
        // suma total = -15 -7 +10 +4 +8 = 0
        let map = map_from(vec![(a, "-15"), (b, "-7"), (c, "10"), (d, "4"), (e, "8")]);

        let result = recommend_settlements(&map);

        // verificar que cada usuario queda saldado: su balance original
        // es igual a (recibido - pagado)
        for (u, original) in [(a, "-15"), (b, "-7"), (c, "10"), (d, "4"), (e, "8")] {
            let paid = total_from(&result, u);
            let received = total_to(&result, u);
            let net = received - paid;
            assert_eq!(net, bd(original), "usuario no quedo saldado correctamente");
        }
    }

    #[test]
    fn no_self_settlements() {
        let a = user();
        let b = user();
        let map = map_from(vec![(a, "-10"), (b, "10")]);
        let result = recommend_settlements(&map);
        for s in &result {
            assert_ne!(s.from, s.to, "un settlement no deberia tener from == to");
        }
    }

    #[test]
    fn all_amounts_are_strictly_positive() {
        let a = user();
        let b = user();
        let c = user();
        let map = map_from(vec![(a, "-3"), (b, "-7"), (c, "10")]);
        let result = recommend_settlements(&map);
        for s in &result {
            assert!(s.amount > BigDecimal::zero(), "settlement con monto <= 0");
        }
    }
}
