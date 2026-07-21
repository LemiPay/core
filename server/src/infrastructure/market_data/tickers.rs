use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

use serde::Deserialize;

/// Global ticker map from `config/coingecko_tickers.toml`, loaded once.
static TICKER_MAP: OnceLock<TickerMap> = OnceLock::new();

/// CoinGecko URL category + page slug + API id for one symbol.
#[derive(Debug, Clone)]
pub struct TickerEntry {
    /// URL path segment: coins | stocks | commodities
    pub tag: String,
    /// Web slug: /en/{tag}/{page}
    pub page: String,
    /// CoinGecko `/simple/price` id (often a tokenized stock coin id)
    pub api_id: String,
}

#[derive(Debug, Clone, Default)]
pub struct TickerMap {
    by_symbol: HashMap<String, TickerEntry>,
    source_path: Option<PathBuf>,
}

/// Accept either `"bitcoin"` or `{ page = "apple", api = "apple-ondo-tokenized-stock" }`.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum TickerValue {
    Simple(String),
    Detailed {
        page: String,
        #[serde(default)]
        api: Option<String>,
    },
}

impl TickerValue {
    fn into_parts(self) -> (String, String) {
        match self {
            TickerValue::Simple(s) => {
                let s = s.trim().to_string();
                (s.clone(), s)
            }
            TickerValue::Detailed { page, api } => {
                let page = page.trim().to_string();
                let api_id = api
                    .map(|a| a.trim().to_string())
                    .filter(|a| !a.is_empty())
                    .unwrap_or_else(|| page.clone());
                (page, api_id)
            }
        }
    }
}

#[derive(Debug, Deserialize, Default)]
struct TickersFile {
    #[serde(default)]
    coins: HashMap<String, TickerValue>,
    #[serde(default)]
    stocks: HashMap<String, TickerValue>,
    #[serde(default)]
    commodities: HashMap<String, TickerValue>,
    /// Legacy flat map → coins
    #[serde(default)]
    tickers: HashMap<String, TickerValue>,
}

impl TickerMap {
    pub fn global() -> &'static TickerMap {
        TICKER_MAP.get_or_init(load_ticker_map)
    }

    pub fn entry(&self, symbol: &str) -> Option<&TickerEntry> {
        self.by_symbol.get(&symbol.to_uppercase())
    }

    pub fn coingecko_id(&self, symbol: &str) -> Option<&str> {
        self.entry(symbol).map(|e| e.api_id.as_str())
    }

    /// API id for /simple/price
    pub fn resolve(&self, symbol: &str, external_id: &str) -> Option<String> {
        if let Some(e) = self.entry(symbol) {
            return Some(e.api_id.clone());
        }
        let ext = external_id.trim();
        if !ext.is_empty() {
            return Some(ext.to_string());
        }
        None
    }

    /// `https://www.coingecko.com/en/{tag}/{page}` e.g. stocks/nvidia
    pub fn price_page_url(&self, symbol: &str, external_id: &str) -> String {
        if let Some(e) = self.entry(symbol) {
            return format!("https://www.coingecko.com/en/{}/{}", e.tag, e.page);
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
}

fn candidate_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    if let Ok(p) = std::env::var("COINGECKO_TICKERS_FILE") {
        paths.push(PathBuf::from(p));
    }
    paths.push(PathBuf::from("../../config/coingecko_tickers.toml"));
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
    section: HashMap<String, TickerValue>,
) {
    for (k, v) in section {
        let sym = k.trim().to_uppercase();
        if sym.is_empty() {
            continue;
        }
        let (page, api_id) = v.into_parts();
        if page.is_empty() || api_id.is_empty() {
            continue;
        }
        by_symbol.insert(
            sym,
            TickerEntry {
                tag: tag.to_string(),
                page,
                api_id,
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
                    insert_section(&mut by_symbol, "coins", file.tickers);

                    println!(
                        "CoinGecko tickers: loaded {} symbols from {}",
                        by_symbol.len(),
                        path.display()
                    );
                    for (sym, e) in &by_symbol {
                        if e.page != e.api_id {
                            println!("  {sym}: page={}/{}  api={}", e.tag, e.page, e.api_id);
                        }
                    }
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

    eprintln!("CoinGecko tickers: no config file found. Using empty map.");
    TickerMap::default()
}
