use crate::core::balances_map::BalancesMap;
use crate::models::transaction::{MyTransactionType, Transaction};
use crate::schema::transaction::amount;
use bigdecimal::{BigDecimal, Zero};
use std::collections::HashMap;
use uuid::Uuid;
use crate::models::expense::Expense;

/// Función central de LemiPay que reconstruye el estado actual de los balances.
///
/// # ¡Importante sobre los Usuarios!
/// El parámetro `users_id` **DEBE** contener a todos los usuarios históricos del grupo
/// (tanto los miembros activos actuales como los que ya abandonaron el grupo) que
/// tengan alguna transacción asociada en el historial.
/// Si una transacción hace referencia a un usuario que no está en esta lista,
/// el programa entrará en pánico para evitar corrupción de datos.
pub fn core(users_id: Vec<Uuid>, transactions: Vec<Transaction>, expenses: Vec<Expense>) -> BalancesMap {
    let balances = create_empty_map(users_id);

    let balances = read_all_tx(transactions, balances);
    read_all_expenses(expenses, balances)
}

fn read_all_tx(transactions: Vec<Transaction>, mut balances: BalancesMap) -> BalancesMap {
    for tx in transactions {
        balances = match tx.tx_type {
            MyTransactionType::Deposit => balances
                .add_balance_to_user(tx.user_id, tx.amount)
                .expect("Error de Integridad: Usuario no encontrado."),
            MyTransactionType::Withdraw => balances
                .remove_to_all(tx.amount)
                .expect("Error de Integridad: Fondos insuficientes."),
            _ => balances,
        };
    }
    balances
}
fn read_all_expenses(expenses: Vec<Expense>, mut balances: BalancesMap) -> BalancesMap {
    for expense in expenses {
        balances = balances
            .add_balance_to_user(expense.user_id, expense.amount.clone())
            .expect("Error de Integridad: Usuario de expense no encontrado.");

        balances = balances
            .remove_to_all(expense.amount)
            .expect("Error de Integridad: Fondos insuficientes en expense.");
    }
    balances
}

pub fn create_empty_map(users_id: Vec<Uuid>) -> BalancesMap {
    let mut balances = HashMap::with_capacity(users_id.len());

    for id in users_id {
        balances.insert(id, BigDecimal::zero());
    }

    BalancesMap::new(balances, BigDecimal::zero())
}
