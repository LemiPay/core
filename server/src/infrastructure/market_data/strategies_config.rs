use std::collections::HashMap;
use std::str::FromStr;

use bigdecimal::BigDecimal;
use uuid::Uuid;

use super::tickers::TickerMap;

/// One strategy definition (catalog entry synced into Postgres on startup).
#[derive(Debug, Clone)]
pub struct StrategyDefinition {
    pub id: Option<Uuid>,
    pub name: String,
    pub description: String,
    pub risk_level: String,
    pub duration_days: i32,
    pub valuation_mode: String,
    pub category: String,
    pub ragequit_fee_bps: i32,
    pub expected_return_percentage: BigDecimal,
    /// Leverage multiplier (1 = no leverage)
    pub leverage: i32,
    /// symbol (uppercase) → weight_bps
    pub allocation: HashMap<String, i32>,
}

impl StrategyDefinition {
    pub fn validate(&self, tickers: &TickerMap) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("strategy name is empty".into());
        }
        if self.duration_days <= 0 {
            return Err(format!(
                "strategy '{}': duration_days must be > 0",
                self.name
            ));
        }
        if !matches!(self.risk_level.as_str(), "low" | "medium" | "high") {
            return Err(format!(
                "strategy '{}': risk_level must be low|medium|high",
                self.name
            ));
        }
        if !matches!(self.valuation_mode.as_str(), "simulated" | "mark_to_market") {
            return Err(format!(
                "strategy '{}': valuation_mode must be simulated|mark_to_market",
                self.name
            ));
        }
        if self.leverage < 1 {
            return Err(format!(
                "strategy '{}': leverage must be >= 1 (got {})",
                self.name, self.leverage
            ));
        }
        if self.valuation_mode == "mark_to_market" {
            if self.allocation.is_empty() {
                return Err(format!(
                    "strategy '{}': mark_to_market requires non-empty allocation",
                    self.name
                ));
            }
            let sum: i32 = self.allocation.values().sum();
            if sum != 10_000 {
                return Err(format!(
                    "strategy '{}': allocation weights sum to {sum}, expected 10000",
                    self.name
                ));
            }
            for (sym, w) in &self.allocation {
                if *w <= 0 {
                    return Err(format!(
                        "strategy '{}': weight for {sym} must be > 0",
                        self.name
                    ));
                }
                if tickers.coingecko_id(sym).is_none() {
                    return Err(format!(
                        "strategy '{}': symbol {sym} is not in hardcoded ticker catalog",
                        self.name
                    ));
                }
            }
        } else if !self.allocation.is_empty() {
            eprintln!(
                "strategy '{}': allocation ignored for simulated valuation_mode",
                self.name
            );
        }
        Ok(())
    }
}

// =============================================================================
// POST-DEMO: DELETE THIS HARDCODED CATALOG
// -----------------------------------------------------------------------------
// Temporary workaround: Azure/container could not read investment_strategies.toml
// at runtime, so the catalog is inlined here. After the demo, restore loading from
// config/investment_strategies.toml (or move catalog fully to DB/admin UI) and
// delete this block + hardcoded_strategy_catalog().
// =============================================================================

fn pct(v: f64) -> BigDecimal {
    BigDecimal::from_str(&format!("{v}")).unwrap_or_else(|_| BigDecimal::from(0))
}

fn alloc(pairs: &[(&str, i32)]) -> HashMap<String, i32> {
    pairs.iter().map(|(s, w)| (s.to_uppercase(), *w)).collect()
}

fn sid(s: &str) -> Option<Uuid> {
    Some(Uuid::parse_str(s).expect("hardcoded strategy id must be valid UUID"))
}

/// POST-DEMO: DELETE — hardcoded catalog (was config/investment_strategies.toml).
fn hardcoded_strategy_catalog() -> Vec<StrategyDefinition> {
    vec![
        // ── Simulated (legacy paper formula) ───────────────────────────────
        StrategyDefinition {
            id: sid("c0000001-0000-4000-8000-000000000001"),
            name: "Fondo Común Lemipay".into(),
            description: "Bajo riesgo, retorno estable y predecible".into(),
            risk_level: "low".into(),
            duration_days: 30,
            valuation_mode: "simulated".into(),
            category: "simulated".into(),
            ragequit_fee_bps: 200,
            expected_return_percentage: pct(3.0),
            leverage: 1,
            allocation: HashMap::new(),
        },
        StrategyDefinition {
            id: sid("c0000001-0000-4000-8000-000000000002"),
            name: "TOP 100 ARG".into(),
            description: "Riesgo medio, buen retorno balanceado".into(),
            risk_level: "medium".into(),
            duration_days: 60,
            valuation_mode: "simulated".into(),
            category: "simulated".into(),
            ragequit_fee_bps: 200,
            expected_return_percentage: pct(7.5),
            leverage: 1,
            allocation: HashMap::new(),
        },
        StrategyDefinition {
            id: sid("c0000001-0000-4000-8000-000000000003"),
            name: "Michael Saylor".into(),
            description: "Alto riesgo, alto retorno potencial".into(),
            risk_level: "high".into(),
            duration_days: 90,
            valuation_mode: "simulated".into(),
            category: "simulated".into(),
            ragequit_fee_bps: 200,
            expected_return_percentage: pct(15.0),
            leverage: 1,
            allocation: HashMap::new(),
        },
        // ── Mark-to-market (CoinGecko prices) ──────────────────────────────
        StrategyDefinition {
            id: sid("b0000001-0000-4000-8000-000000000001"),
            name: "Crypto Bluechips".into(),
            description: "Bluechips crypto: BTC, ETH, SOL.".into(),
            risk_level: "medium".into(),
            duration_days: 30,
            valuation_mode: "mark_to_market".into(),
            category: "crypto".into(),
            ragequit_fee_bps: 200,
            expected_return_percentage: pct(0.0),
            leverage: 1,
            allocation: alloc(&[("BTC", 5000), ("ETH", 3000), ("SOL", 2000)]),
        },
        StrategyDefinition {
            id: sid("b0000001-0000-4000-8000-000000000002"),
            name: "Crypto Aggressive".into(),
            description: "Crypto de mayor volatilidad: SOL, ETH, BTC, DOGE.".into(),
            risk_level: "high".into(),
            duration_days: 45,
            valuation_mode: "mark_to_market".into(),
            category: "crypto".into(),
            ragequit_fee_bps: 200,
            expected_return_percentage: pct(0.0),
            leverage: 1,
            allocation: alloc(&[("SOL", 4000), ("ETH", 3000), ("BTC", 2000), ("DOGE", 1000)]),
        },
        StrategyDefinition {
            id: sid("b0000001-0000-4000-8000-000000000003"),
            name: "Tech Equity".into(),
            description: "Stocks (paper): AAPL, MSFT, NVDA, GOOGL y SPCX.".into(),
            risk_level: "medium".into(),
            duration_days: 60,
            valuation_mode: "mark_to_market".into(),
            category: "stocks".into(),
            ragequit_fee_bps: 200,
            expected_return_percentage: pct(0.0),
            leverage: 1,
            allocation: alloc(&[
                ("AAPL", 3000),
                ("MSFT", 2500),
                ("NVDA", 2000),
                ("GOOGL", 1500),
                ("SPCX", 1000),
            ]),
        },
        StrategyDefinition {
            id: sid("b0000001-0000-4000-8000-000000000004"),
            name: "Gold Reserve".into(),
            description: "Exposición 100% a oro (commodities en CoinGecko).".into(),
            risk_level: "low".into(),
            duration_days: 30,
            valuation_mode: "mark_to_market".into(),
            category: "rwa".into(),
            ragequit_fee_bps: 100,
            expected_return_percentage: pct(0.0),
            leverage: 1,
            allocation: alloc(&[("GOLD", 10000)]),
        },
        StrategyDefinition {
            id: sid("b0000001-0000-4000-8000-000000000005"),
            name: "Balanced Mix".into(),
            description: "Mix crypto + stocks + gold: BTC, ETH, AAPL, SPCX, GOLD.".into(),
            risk_level: "medium".into(),
            duration_days: 45,
            valuation_mode: "mark_to_market".into(),
            category: "mixed".into(),
            ragequit_fee_bps: 200,
            expected_return_percentage: pct(0.0),
            leverage: 1,
            allocation: alloc(&[
                ("BTC", 3000),
                ("ETH", 1500),
                ("AAPL", 2000),
                ("SPCX", 1500),
                ("GOLD", 2000),
            ]),
        },
        // ── Leveraged ──────────────────────────────────────────────────────
        StrategyDefinition {
            id: sid("b0000001-0000-4000-8000-000000000010"),
            name: "Crypto Bluechips 2x".into(),
            description:
                "Misma basket BTC/ETH/SOL con leverage 2x. Liquidación si el basket cae ≥ 50%."
                    .into(),
            risk_level: "high".into(),
            duration_days: 30,
            valuation_mode: "mark_to_market".into(),
            category: "crypto".into(),
            ragequit_fee_bps: 200,
            expected_return_percentage: pct(0.0),
            leverage: 2,
            allocation: alloc(&[("BTC", 5000), ("ETH", 3000), ("SOL", 2000)]),
        },
        StrategyDefinition {
            id: sid("b0000001-0000-4000-8000-000000000011"),
            name: "Crypto Aggressive 5x".into(),
            description: "Basket agresiva con leverage 5x. Liquidación si el basket cae ≥ 20%."
                .into(),
            risk_level: "high".into(),
            duration_days: 30,
            valuation_mode: "mark_to_market".into(),
            category: "crypto".into(),
            ragequit_fee_bps: 300,
            expected_return_percentage: pct(0.0),
            leverage: 5,
            allocation: alloc(&[("SOL", 2000), ("ETH", 3000), ("BTC", 4000), ("DOGE", 1000)]),
        },
        StrategyDefinition {
            id: sid("b0000001-0000-4000-8000-000000000012"),
            name: "Tech Equity 3x".into(),
            description: "Stocks tokenizadas con leverage 3x. Liquidación si el basket cae ≥ ~33%."
                .into(),
            risk_level: "high".into(),
            duration_days: 45,
            valuation_mode: "mark_to_market".into(),
            category: "stocks".into(),
            ragequit_fee_bps: 250,
            expected_return_percentage: pct(0.0),
            leverage: 3,
            allocation: alloc(&[
                ("AAPL", 3000),
                ("MSFT", 2500),
                ("NVDA", 2000),
                ("GOOGL", 1500),
                ("SPCX", 1000),
            ]),
        },
        StrategyDefinition {
            id: sid("b0000001-0000-4000-8000-000000000013"),
            name: "Crazy Mix 50x".into(),
            description: "Mix crypto + stocks con leverage 50x. Liquidación si el basket cae ≥ 2%."
                .into(),
            risk_level: "high".into(),
            duration_days: 30,
            valuation_mode: "mark_to_market".into(),
            category: "mixed".into(),
            ragequit_fee_bps: 250,
            expected_return_percentage: pct(0.0),
            leverage: 50,
            allocation: alloc(&[
                ("AAPL", 3000),
                ("MSFT", 2500),
                ("BTC", 2000),
                ("ETH", 1000),
                ("SPCX", 1500),
            ]),
        },
    ]
}

// =============================================================================
// END POST-DEMO HARDCODED CATALOG
// =============================================================================

/// Load strategy definitions.
///
/// POST-DEMO: restore TOML load from config/investment_strategies.toml; today
/// this returns the inlined catalog (Azure/container file-read workaround).
pub fn load_strategy_definitions() -> Result<(Vec<StrategyDefinition>, Option<String>), String> {
    let tickers = TickerMap::global();
    let raw = hardcoded_strategy_catalog();
    let mut out = Vec::with_capacity(raw.len());
    let mut skipped = 0usize;

    for def in raw {
        if let Err(e) = def.validate(tickers) {
            eprintln!("Investment strategies: skip '{}': {e}", def.name);
            skipped += 1;
            continue;
        }
        out.push(def);
    }

    // POST-DEMO: change log source label back to file path when TOML returns.
    println!(
        "Investment strategies: loaded {} from hardcoded catalog ({} skipped) [POST-DEMO: remove hardcode]",
        out.len(),
        skipped
    );
    Ok((out, Some("hardcoded://strategies_config.rs".into())))
}

/// Map CoinGecko config section tag → asset.kind
pub fn asset_kind_for_symbol(symbol: &str) -> &'static str {
    if let Some(e) = TickerMap::global().entry(symbol) {
        return match e.tag.as_str() {
            "stocks" => "tokenized_stock",
            "commodities" => "rwa",
            _ => "crypto",
        };
    }
    "crypto"
}

pub fn asset_name_for_symbol(symbol: &str) -> String {
    match symbol.to_uppercase().as_str() {
        "BTC" => "Bitcoin".into(),
        "ETH" => "Ethereum".into(),
        "SOL" => "Solana".into(),
        "DOGE" => "Dogecoin".into(),
        "AAPL" => "Apple".into(),
        "MSFT" => "Microsoft".into(),
        "NVDA" => "NVIDIA".into(),
        "GOOGL" => "Alphabet".into(),
        "SPCX" => "SpaceX".into(),
        "GOLD" => "Gold".into(),
        other => other.to_string(),
    }
}

/// POST-DEMO: was path to TOML; now a fixed label for logs only.
pub fn strategies_path_hint() -> String {
    "hardcoded://strategies_config.rs (POST-DEMO: restore TOML)".into()
}
