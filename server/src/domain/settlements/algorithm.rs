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
