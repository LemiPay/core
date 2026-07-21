use std::collections::HashMap;
use std::sync::OnceLock;

/// Global ticker map (hardcoded catalog), loaded once.
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

    pub fn len(&self) -> usize {
        self.by_symbol.len()
    }
}

// =============================================================================
// POST-DEMO: DELETE THIS HARDCODED CATALOG
// -----------------------------------------------------------------------------
// Temporary workaround: Azure/container ships only the binary (no config/), so
// coingecko_tickers.toml cannot be read at runtime. Catalog is inlined here.
// After the demo, restore loading from config/coingecko_tickers.toml (or move
// fully to DB/admin UI) and delete this block.
// =============================================================================

fn entry(tag: &str, page: &str, api_id: &str) -> TickerEntry {
    TickerEntry {
        tag: tag.to_string(),
        page: page.to_string(),
        api_id: api_id.to_string(),
    }
}

/// POST-DEMO: DELETE — hardcoded ticker catalog (was config/coingecko_tickers.toml).
fn hardcoded_ticker_catalog() -> HashMap<String, TickerEntry> {
    let mut m = HashMap::new();

    // coins: page = api = slug
    for (sym, slug) in [
        ("BTC", "bitcoin"),
        ("ETH", "ethereum"),
        ("SOL", "solana"),
        ("DOGE", "dogecoin"),
    ] {
        m.insert(sym.to_string(), entry("coins", slug, slug));
    }

    // stocks: page = /en/stocks/{page} | api = CoinGecko coin id for /simple/price
    for (sym, page, api) in [
        ("AAPL", "apple", "apple-ondo-tokenized-stock"),
        ("MSFT", "microsoft", "microsoft-ondo-tokenized-stock"),
        ("NVDA", "nvidia", "nvidia-ondo-tokenized-stock"),
        ("GOOGL", "alphabet", "alphabet-class-a-ondo-tokenized-stock"),
        ("SPCX", "spacex", "spacex-bstocks-tokenized-stock"),
    ] {
        m.insert(sym.to_string(), entry("stocks", page, api));
    }

    // commodities
    m.insert("GOLD".to_string(), entry("commodities", "gold", "pax-gold"));

    m
}

// =============================================================================
// END POST-DEMO HARDCODED CATALOG
// =============================================================================

fn load_ticker_map() -> TickerMap {
    let by_symbol = hardcoded_ticker_catalog();
    println!(
        "CoinGecko tickers: loaded {} symbols from hardcoded catalog [POST-DEMO: remove hardcode]",
        by_symbol.len()
    );
    for (sym, e) in &by_symbol {
        if e.page != e.api_id {
            println!("  {sym}: page={}/{}  api={}", e.tag, e.page, e.api_id);
        }
    }
    TickerMap { by_symbol }
}
