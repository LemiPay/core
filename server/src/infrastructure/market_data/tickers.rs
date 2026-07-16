use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use serde::Deserialize;

/// Global ticker map from `config/coingecko_tickers.toml`, loaded once.
static TICKER_MAP: OnceLock<TickerMap> = OnceLock::new();

/// CoinGecko URL category + API id for one symbol.
#[derive(Debug, Clone)]
pub struct TickerEntry {
    /// URL path segment: coins | stocks | commodities
    pub tag: String,
    /// CoinGecko slug/id, e.g. "nvidia", "bitcoin"
    pub id: String,
}

#[derive(Debug, Clone, Default)]
pub struct TickerMap {
    /// Uppercase symbol → entry
    by_symbol: HashMap<String, TickerEntry>,
    source_path: Option<PathBuf>,
}

/// TOML shape:
/// ```toml
/// [coins]
/// BTC = "bitcoin"
/// [stocks]
/// NVDA = "nvidia"
/// [commodities]
/// GOLD = "gold"
/// ```
#[derive(Debug, Deserialize, Default)]
struct TickersFile {
    #[serde(default)]
    coins: HashMap<String, String>,
    #[serde(default)]
    stocks: HashMap<String, String>,
    #[serde(default)]
    commodities: HashMap<String, String>,
    /// Legacy flat map (back-compat)
    #[serde(default)]
    tickers: HashMap<String, String>,
}

impl TickerMap {
    pub fn global() -> &'static TickerMap {
        TICKER_MAP.get_or_init(load_ticker_map)
    }

    pub fn entry(&self, symbol: &str) -> Option<&TickerEntry> {
        self.by_symbol.get(&symbol.to_uppercase())
    }

    pub fn coingecko_id(&self, symbol: &str) -> Option<&str> {
        self.entry(symbol).map(|e| e.id.as_str())
    }

    /// Resolve API id: config by symbol, else external_id fallback.
    pub fn resolve(&self, symbol: &str, external_id: &str) -> Option<String> {
        if let Some(e) = self.entry(symbol) {
            return Some(e.id.clone());
        }
        let ext = external_id.trim();
        if !ext.is_empty() {
            return Some(ext.to_string());
        }
        None
    }

    /// `https://www.coingecko.com/en/{tag}/{id}` e.g. stocks/nvidia
    pub fn price_page_url(&self, symbol: &str, external_id: &str) -> String {
        if let Some(e) = self.entry(symbol) {
            return format!("https://www.coingecko.com/en/{}/{}", e.tag, e.id);
        }
        let id = if external_id.trim().is_empty() {
            symbol.to_lowercase()
        } else {
            external_id.trim().to_string()
        };
        format!("https://www.coingecko.com/en/coins/{id}")
    }

    pub fn source_path(&self) -> Option<&Path> {
        self.source_path.as_deref()
    }

    pub fn len(&self) -> usize {
        self.by_symbol.len()
    }

    pub fn symbols(&self) -> Vec<String> {
        let mut v: Vec<String> = self.by_symbol.keys().cloned().collect();
        v.sort();
        v
    }
}

fn candidate_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    if let Ok(p) = std::env::var("COINGECKO_TICKERS_FILE") {
        paths.push(PathBuf::from(p));
    }
    paths.push(PathBuf::from("config/coingecko_tickers.toml"));
    paths.push(PathBuf::from("server/config/coingecko_tickers.toml"));
    paths.push(PathBuf::from(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/config/coingecko_tickers.toml"
    )));
    paths
}

fn insert_section(
    by_symbol: &mut HashMap<String, TickerEntry>,
    tag: &str,
    section: HashMap<String, String>,
) {
    for (k, v) in section {
        let sym = k.trim().to_uppercase();
        let id = v.trim().to_string();
        if sym.is_empty() || id.is_empty() {
            continue;
        }
        by_symbol.insert(
            sym,
            TickerEntry {
                tag: tag.to_string(),
                id,
            },
        );
    }
}

fn load_ticker_map() -> TickerMap {
    for path in candidate_paths() {
        if !path.is_file() {
            continue;
        }
        match std::fs::read_to_string(&path) {
            Ok(raw) => match toml::from_str::<TickersFile>(&raw) {
                Ok(file) => {
                    let mut by_symbol = HashMap::new();
                    insert_section(&mut by_symbol, "coins", file.coins);
                    insert_section(&mut by_symbol, "stocks", file.stocks);
                    insert_section(&mut by_symbol, "commodities", file.commodities);
                    // Legacy [tickers] → treat as coins
                    insert_section(&mut by_symbol, "coins", file.tickers);

                    println!(
                        "CoinGecko tickers: loaded {} symbols from {} ({})",
                        by_symbol.len(),
                        path.display(),
                        by_symbol.keys().cloned().collect::<Vec<_>>().join(", ")
                    );
                    return TickerMap {
                        by_symbol,
                        source_path: Some(path),
                    };
                }
                Err(e) => {
                    eprintln!("CoinGecko tickers: failed to parse {}: {e}", path.display());
                }
            },
            Err(e) => {
                eprintln!("CoinGecko tickers: cannot read {}: {e}", path.display());
            }
        }
    }

    eprintln!(
        "CoinGecko tickers: no config file found (set COINGECKO_TICKERS_FILE or add config/coingecko_tickers.toml). Using empty map."
    );
    TickerMap::default()
}
