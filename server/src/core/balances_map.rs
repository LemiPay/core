use bigdecimal::{BigDecimal, RoundingMode, Zero};
use std::collections::HashMap;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug)]
pub struct BalancesMap {
    balances: HashMap<Uuid, BigDecimal>,
    group_balance: BigDecimal,
}
impl BalancesMap {
    pub fn new(balances: HashMap<Uuid, BigDecimal>, group_balance: BigDecimal) -> Self {
        Self {
            balances,
            group_balance,
        }
    }
    // Fíjate en el Result de acá arriba
    pub fn add_balance_to_user(
        &self,
        user_id: Uuid,
        balance: BigDecimal,
    ) -> Result<BalancesMap, &'static str> {
        let mut new_balances: HashMap<Uuid, BigDecimal> = self.balances.clone();

        // Validación clave que hace que devuelva Err
        if !new_balances.contains_key(&user_id) {
            return Err("Operación denegada: El usuario no existe en este grupo.");
        }

        new_balances.entry(user_id).and_modify(|previous_balance| {
            *previous_balance += balance.clone();
        });

        let new_group_balance = self.group_balance.clone() + balance;

        // Retornamos con Ok()
        Ok(BalancesMap::new(new_balances, new_group_balance))
    }
    pub fn get_user_balance(&self, user_id: &Uuid) -> Option<&BigDecimal> {
        self.balances.get(user_id)
    }
    pub fn get_group_balance(&self) -> &BigDecimal {
        &self.group_balance
    }
    // pub fn get_all_balances(&self) -> &HashMap<Uuid, BigDecimal> {
    //     &self.balances
    // }

    /// Resta un balance distribuido entre todos los usuarios.
    /// Retorna Ok(BalancesMap) si la operación es válida,
    /// o Err si se intenta sacar más del balance disponible en el grupo.
    pub fn remove_to_all(&self, total_balance: BigDecimal) -> Result<BalancesMap, &'static str> {
        // 1. Validación anti-sobregiro del grupo
        if total_balance > self.group_balance {
            return Err("Operación denegada: El balance a restar supera los fondos del grupo.");
        }

        let n = self.balances.len();

        // 2. Prevención de división por cero
        if n == 0 {
            return Ok(BalancesMap::new(
                self.balances.clone(),
                self.group_balance.clone(),
            ));
        }
        //TODO CREAR ESTRUCTURA QUE SE ENCARGUE DE ESTO Y MANTENGA LOGICA DE DECIMALES ESTILO ETH

        // 3. Configuración de precisión (18 decimales)
        let escala = 18;
        let unidad_minima = BigDecimal::from_str("0.000000000000000001").unwrap();
        let n_decimal = BigDecimal::from(n as u64);

        // 4. División base truncada
        let amount_per_user = (total_balance.clone() / n_decimal.clone())
            .with_scale_round(escala, RoundingMode::Down);

        let total_deducted = amount_per_user.clone() * n_decimal;
        let mut remanente = total_balance.clone() - total_deducted;

        let mut new_balances = self.balances.clone();

        for user_balance in new_balances.values_mut() {
            let mut deduccion = amount_per_user.clone();

            if remanente > BigDecimal::zero() {
                deduccion += unidad_minima.clone();
                remanente -= unidad_minima.clone();
            }

            *user_balance -= deduccion;
        }

        let new_group_balance = self.group_balance.clone() - total_balance;

        Ok(BalancesMap::new(new_balances, new_group_balance))
    }
}
