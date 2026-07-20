use bigdecimal::{BigDecimal, Zero};

/// Paper leverage helpers.
///
/// margin M, leverage L, entry exposure E0 = M*L, current exposure E_t:
///   equity = M + (E_t - E0) = E_t - M*(L-1)
/// Liquidate when equity <= 0  ⇔  basket return <= -1/L
pub struct LeverageMath;

impl LeverageMath {
    pub fn entry_exposure(margin: &BigDecimal, leverage: i32) -> BigDecimal {
        let l = if leverage < 1 { 1 } else { leverage };
        margin * BigDecimal::from(l)
    }

    /// Recoverable equity (what current_value stores).
    pub fn equity(
        margin: &BigDecimal,
        entry_exposure: &BigDecimal,
        current_exposure: &BigDecimal,
    ) -> BigDecimal {
        margin + (current_exposure - entry_exposure)
    }

    /// Liquidation when equity cannot cover the position (equity ≤ 0).
    pub fn is_liquidatable(equity: &BigDecimal) -> bool {
        *equity <= BigDecimal::zero()
    }

    /// Exposure level at which equity hits 0: M * (L - 1)
    pub fn liquidation_threshold(margin: &BigDecimal, leverage: i32) -> BigDecimal {
        let l = if leverage < 1 { 1 } else { leverage };
        if l <= 1 {
            return BigDecimal::zero();
        }
        margin * BigDecimal::from(l - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entry_exposure_5x() {
        let m = BigDecimal::from(100);
        assert_eq!(LeverageMath::entry_exposure(&m, 5), BigDecimal::from(500));
    }

    #[test]
    fn equity_flat() {
        let m = BigDecimal::from(100);
        let e0 = BigDecimal::from(500);
        let et = BigDecimal::from(500);
        assert_eq!(LeverageMath::equity(&m, &e0, &et), BigDecimal::from(100));
    }

    #[test]
    fn equity_down_20pct_on_5x_is_zero() {
        // basket -20% on 5x exposure: E_t = 400, equity = 100 + (400-500) = 0
        let m = BigDecimal::from(100);
        let e0 = BigDecimal::from(500);
        let et = BigDecimal::from(400);
        let eq = LeverageMath::equity(&m, &e0, &et);
        assert_eq!(eq, BigDecimal::from(0));
        assert!(LeverageMath::is_liquidatable(&eq));
    }

    #[test]
    fn equity_down_10pct_on_5x_survives() {
        let m = BigDecimal::from(100);
        let e0 = BigDecimal::from(500);
        let et = BigDecimal::from(450);
        let eq = LeverageMath::equity(&m, &e0, &et);
        assert_eq!(eq, BigDecimal::from(50));
        assert!(!LeverageMath::is_liquidatable(&eq));
    }

    #[test]
    fn one_x_no_liq_at_half() {
        let m = BigDecimal::from(100);
        let e0 = BigDecimal::from(100);
        let et = BigDecimal::from(50);
        let eq = LeverageMath::equity(&m, &e0, &et);
        assert_eq!(eq, BigDecimal::from(50));
        assert!(!LeverageMath::is_liquidatable(&eq));
    }
}
