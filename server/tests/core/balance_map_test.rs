#[path = "../../src/core/balances_map.rs"]
mod balances_map;
#[cfg(test)]
mod tests {
    use super::balances_map::BalancesMap;
    use super::*;
    use bigdecimal::BigDecimal;
    use std::collections::HashMap;
    use std::str::FromStr;
    use uuid::Uuid;

    // --- FUNCIÓN AUXILIAR ---
    // Nos ayuda a crear BigDecimals rápido a partir de strings para que
    // los tests sean más fáciles de leer, sin ensuciar todo con unwrap().
    fn bd(val: &str) -> BigDecimal {
        BigDecimal::from_str(val).unwrap()
    }

    // Configuración base reutilizable para los tests
    fn setup_mapa_basico() -> (BalancesMap, Uuid, Uuid, Uuid) {
        let u1 = Uuid::new_v4();
        let u2 = Uuid::new_v4();
        let u3 = Uuid::new_v4();

        let mut balances = HashMap::new();
        balances.insert(u1, bd("10.0"));
        balances.insert(u2, bd("10.0"));
        balances.insert(u3, bd("10.0"));

        let mapa = BalancesMap::new(balances, bd("30.0"));
        (mapa, u1, u2, u3)
    }

    #[test]
    fn test_inicializacion_y_getters() {
        let (mapa, u1, _, _) = setup_mapa_basico();

        // Verificamos que el balance global se asigne bien
        assert_eq!(mapa.get_group_balance(), &bd("30.0"));

        // Verificamos que podamos leer a un usuario específico
        assert_eq!(mapa.get_user_balance(&u1), Some(&bd("10.0")));

        // Verificamos que un usuario inexistente devuelva None
        let u_fantasma = Uuid::new_v4();
        assert_eq!(mapa.get_user_balance(&u_fantasma), None);
    }

    #[test]
    fn test_add_balance_to_user_existente() {
        let (mapa, u1, _, _) = setup_mapa_basico();

        // Le sumamos 5.5 al usuario 1
        let mapa_actualizado = mapa.add_balance_to_user(u1, bd("5.5"));

        // El usuario debería tener 15.5
        assert_eq!(mapa_actualizado.get_user_balance(&u1), Some(&bd("15.5")));

        // El grupo debería haber aumentado a 35.5
        assert_eq!(mapa_actualizado.get_group_balance(), &bd("35.5"));

        // Verificamos que la inmutabilidad funcionó: el mapa original no cambió
        assert_eq!(mapa.get_group_balance(), &bd("30.0"));
    }

    #[test]
    fn test_remove_to_all_division_exacta() {
        let (mapa, u1, u2, u3) = setup_mapa_basico();

        // Quitamos 15 al grupo (exactamente 5 por usuario)
        let resultado = mapa.remove_to_all(bd("15.0"));
        assert!(resultado.is_ok());
        let mapa_nuevo = resultado.unwrap();

        // El grupo debería quedar en 15 (30 - 15)
        assert_eq!(mapa_nuevo.get_group_balance(), &bd("15.0"));

        // Cada usuario debería quedar con 5 (10 - 5)
        assert_eq!(mapa_nuevo.get_user_balance(&u1), Some(&bd("5.0")));
        assert_eq!(mapa_nuevo.get_user_balance(&u2), Some(&bd("5.0")));
        assert_eq!(mapa_nuevo.get_user_balance(&u3), Some(&bd("5.0")));
    }

    #[test]
    fn test_remove_to_all_error_sobregiro() {
        let (mapa, _, _, _) = setup_mapa_basico();

        // Intentamos quitar 40, pero el grupo solo tiene 30
        let resultado = mapa.remove_to_all(bd("40.0"));

        // Verificamos que falle y devuelva el error correcto
        assert!(resultado.is_err());
        assert_eq!(
            resultado.unwrap_err(),
            "Operación denegada: El balance a restar supera los fondos del grupo."
        );
    }

    #[test]
    fn test_remove_to_all_reparto_de_polvo() {
        let (mapa, _, _, _) = setup_mapa_basico();

        // Quitamos 10 al grupo. 10 / 3 = 3.3333... periódico.
        // Esto activará nuestra lógica de remanentes (el polvo cripto).
        let mapa_nuevo = mapa.remove_to_all(bd("10.0")).unwrap();

        // 1. El grupo DEBE quedar exactamente en 20 (30 - 10)
        assert_eq!(mapa_nuevo.get_group_balance(), &bd("20.0"));

        // 2. La suma de los balances de los usuarios DEBE cuadrar perfectamente
        // Inicialmente tenían 30 sumados. Ahora deben sumar 20.
        let suma_usuarios: BigDecimal = mapa_nuevo.get_all_balances().values().cloned().sum();

        // Si el algoritmo del "polvo" fallara, la suma daría 20.000000000000000001 o algo así.
        // Comprobamos que cuadre a la perfección.
        assert_eq!(suma_usuarios, bd("20.0"));

        // 3. Verificamos cómo se repartió comprobando los valores que existen.
        // Como no sabemos qué Uuid recibió el extra (por el orden aleatorio del HashMap),
        // contamos cuántos usuarios tienen el saldo "normal" y cuántos el "penalizado".
        let mut con_polvo = 0;
        let mut sin_polvo = 0;

        for balance in mapa_nuevo.get_all_balances().values() {
            if balance == &bd("6.666666666666666666") {
                con_polvo += 1; // Le restaron 3.33...34 (absorbió el polvo)
            } else if balance == &bd("6.666666666666666667") {
                sin_polvo += 1; // Le restaron 3.33...33
            } else {
                panic!("Un usuario tiene un balance inesperado: {}", balance);
            }
        }

        // De los 3 usuarios, 1 tuvo que absorber el 0...01 extra para que cuadre a 10.
        assert_eq!(con_polvo, 1);
        assert_eq!(sin_polvo, 2);
    }

    #[test]
    fn test_remove_to_all_mapa_vacio() {
        let mapa_vacio = BalancesMap::new(HashMap::new(), bd("100.0"));

        // Intentamos quitar balance sin usuarios
        let resultado = mapa_vacio.remove_to_all(bd("50.0"));
        assert!(resultado.is_ok());

        let mapa_nuevo = resultado.unwrap();
        // El balance del grupo no debió cambiar porque la función aborta al ver n=0
        assert_eq!(mapa_nuevo.get_group_balance(), &bd("100.0"));
    }
}
