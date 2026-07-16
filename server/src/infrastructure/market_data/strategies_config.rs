use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;

use bigdecimal::BigDecimal;
use serde::Deserialize;
use uuid::Uuid;

use super::tickers::TickerMap;

/// One strategy definition from `config/investment_strategies.toml`.
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
    /// symbol (uppercase) → weight_bps
    pub allocation: HashMap<String, i32>,
}

#[derive(Debug, Deserialize)]
struct StrategiesFile {
    strategies: Vec<StrategyToml>,
}

#[derive(Debug, Deserialize)]
struct StrategyToml {
    id: Option<String>,
    name: String,
    description: String,
    risk_level: String,
    duration_days: i32,
    #[serde(default = "default_valuation_mode")]
    valuation_mode: String,
    #[serde(default = "default_category")]
    category: String,
    #[serde(default = "default_ragequit")]
    ragequit_fee_bps: i32,
    #[serde(default)]
    expected_return_percentage: f64,
    /// Inline table: BTC = 5000, ETH = 3000, ...
    #[serde(default)]
    allocation: HashMap<String, i32>,
}

fn default_valuation_mode() -> String {
    "simulated".into()
}
fn default_category() -> String {
    "simulated".into()
}
fn default_ragequit() -> i32 {
    200
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
                        "strategy '{}': symbol {sym} is not in coingecko_tickers.toml",
                        self.name
                    ));
                }
            }
        } else if !self.allocation.is_empty() {
            // Allow but ignore weights for simulated — or warn
            eprintln!(
                "strategy '{}': allocation ignored for simulated valuation_mode",
                self.name
            );
        }
        Ok(())
    }
}

fn candidate_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    if let Ok(p) = std::env::var("INVESTMENT_STRATEGIES_FILE") {
        paths.push(PathBuf::from(p));
    }
    paths.push(PathBuf::from("config/investment_strategies.toml"));
    paths.push(PathBuf::from("server/config/investment_strategies.toml"));
    paths.push(PathBuf::from(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/config/investment_strategies.toml"
    )));
    paths
}

/// Load strategy definitions from TOML. Returns empty vec if file missing (keeps DB seed).
pub fn load_strategy_definitions() -> Result<(Vec<StrategyDefinition>, Option<PathBuf>), String> {
    for path in candidate_paths() {
        if !path.is_file() {
            continue;
        }
        let raw = std::fs::read_to_string(&path)
            .map_err(|e| format!("cannot read {}: {e}", path.display()))?;
        let file: StrategiesFile =
            toml::from_str(&raw).map_err(|e| format!("parse {}: {e}", path.display()))?;

        let tickers = TickerMap::global();
        let mut out = Vec::with_capacity(file.strategies.len());
        for s in file.strategies {
            let id = match s.id.as_deref() {
                Some(raw_id) if !raw_id.trim().is_empty() => Some(
                    Uuid::parse_str(raw_id.trim())
                        .map_err(|e| format!("strategy '{}': invalid id: {e}", s.name))?,
                ),
                _ => None,
            };
            let mut allocation = HashMap::new();
            for (k, v) in s.allocation {
                allocation.insert(k.trim().to_uppercase(), v);
            }
            let pct = BigDecimal::from_str(&format!("{}", s.expected_return_percentage))
                .unwrap_or_else(|_| BigDecimal::from(0));
            let def = StrategyDefinition {
                id,
                name: s.name,
                description: s.description,
                risk_level: s.risk_level,
                duration_days: s.duration_days,
                valuation_mode: s.valuation_mode,
                category: s.category,
                ragequit_fee_bps: s.ragequit_fee_bps,
                expected_return_percentage: pct,
                allocation,
            };
            def.validate(tickers)?;
            out.push(def);
        }
        println!(
            "Investment strategies: loaded {} from {}",
            out.len(),
            path.display()
        );
        return Ok((out, Some(path)));
    }
    eprintln!(
        "Investment strategies: no config file found (config/investment_strategies.toml). Skipping sync."
    );
    Ok((Vec::new(), None))
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

pub fn resolve_strategies_path() -> Option<PathBuf> {
    candidate_paths().into_iter().find(|p| p.is_file())
}

pub fn strategies_path_hint() -> String {
    resolve_strategies_path()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|| "config/investment_strategies.toml".into())
}
