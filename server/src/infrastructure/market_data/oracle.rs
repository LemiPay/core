use std::collections::HashMap;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use async_trait::async_trait;
use bigdecimal::BigDecimal;
use uuid::Uuid;

use super::tickers::TickerMap;

/// Minimal asset identity needed to fetch a USD price.
#[derive(Debug, Clone)]
pub struct AssetPriceRef {
    pub id: Uuid,
    pub symbol: String,
    pub price_provider: String,
    pub external_id: String,
}

#[derive(Debug)]
pub enum PriceOracleError {
    Fetch(String),
    Parse(String),
    MissingPrice(String),
}

impl std::fmt::Display for PriceOracleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fetch(m) => write!(f, "price fetch error: {m}"),
            Self::Parse(m) => write!(f, "price parse error: {m}"),
            Self::MissingPrice(m) => write!(f, "missing price: {m}"),
        }
    }
}

#[async_trait]
pub trait PriceOracle: Send + Sync {
    async fn get_usd_prices(
        &self,
        assets: &[AssetPriceRef],
    ) -> Result<HashMap<Uuid, BigDecimal>, PriceOracleError>;
}

// ── Cache ──────────────────────────────────────────────────────────────────

struct CacheEntry {
    price: BigDecimal,
    at: Instant,
}

struct PriceCache {
    ttl: Duration,
    map: Mutex<HashMap<Uuid, CacheEntry>>,
}

impl PriceCache {
    fn new(ttl_secs: u64) -> Self {
        Self {
            ttl: Duration::from_secs(ttl_secs),
            map: Mutex::new(HashMap::new()),
        }
    }

    fn get(&self, id: Uuid) -> Option<BigDecimal> {
        let map = self.map.lock().ok()?;
        let entry = map.get(&id)?;
        if entry.at.elapsed() < self.ttl {
            Some(entry.price.clone())
        } else {
            None
        }
    }

    fn get_stale(&self, id: Uuid) -> Option<BigDecimal> {
        let map = self.map.lock().ok()?;
        map.get(&id).map(|e| e.price.clone())
    }

    fn put(&self, id: Uuid, price: BigDecimal) {
        if let Ok(mut map) = self.map.lock() {
            map.insert(
                id,
                CacheEntry {
                    price,
                    at: Instant::now(),
                },
            );
        }
    }
}

// ── Mock ───────────────────────────────────────────────────────────────────

/// Offline demo prices (random walk). Bases keyed by ticker symbol.
pub struct MockPriceOracle {
    base: HashMap<String, f64>,
    state: Mutex<HashMap<Uuid, f64>>,
}

impl MockPriceOracle {
    pub fn new() -> Self {
        let mut base = HashMap::new();
        base.insert("BTC".into(), 95_000.0);
        base.insert("ETH".into(), 3_500.0);
        base.insert("SOL".into(), 180.0);
        base.insert("DOGE".into(), 0.18);
        base.insert("AAPL".into(), 210.0);
        base.insert("MSFT".into(), 420.0);
        base.insert("NVDA".into(), 130.0);
        base.insert("GOOGL".into(), 175.0);
        base.insert("GOLD".into(), 2300.0);
        base.insert("SPCX".into(), 120.0);
        base.insert("AAPL".into(), 210.0);
        Self {
            base,
            state: Mutex::new(HashMap::new()),
        }
    }

    fn next_price(&self, asset: &AssetPriceRef) -> BigDecimal {
        let mut state = self.state.lock().unwrap();
        let current = state.entry(asset.id).or_insert_with(|| {
            *self
                .base
                .get(&asset.symbol.to_uppercase())
                .or_else(|| self.base.get(&asset.symbol))
                .unwrap_or(&100.0)
        });
        let jitter = (rand::random::<f64>() - 0.5) * 0.006;
        *current *= 1.0 + jitter;
        if *current <= 0.0 {
            *current = 0.01;
        }
        BigDecimal::from_str(&format!("{:.8}", *current)).unwrap_or_else(|_| BigDecimal::from(1))
    }

    fn prices_sync(&self, assets: &[AssetPriceRef]) -> HashMap<Uuid, BigDecimal> {
        let mut out = HashMap::new();
        for a in assets {
            out.insert(a.id, self.next_price(a));
        }
        out
    }

    pub fn prices_for(&self, assets: &[AssetPriceRef]) -> HashMap<Uuid, BigDecimal> {
        self.prices_sync(assets)
    }
}

impl Default for MockPriceOracle {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl PriceOracle for MockPriceOracle {
    async fn get_usd_prices(
        &self,
        assets: &[AssetPriceRef],
    ) -> Result<HashMap<Uuid, BigDecimal>, PriceOracleError> {
        Ok(self.prices_sync(assets))
    }
}

// ── Live: CoinGecko only (ids from hardcoded ticker catalog) ──────────────

pub struct CoinGeckoPriceOracle {
    client: reqwest::Client,
    cache: PriceCache,
    mock_fallback: MockPriceOracle,
    tickers: &'static TickerMap,
    api_key: Option<String>,
    key_type: String,
}

impl CoinGeckoPriceOracle {
    pub fn new() -> Result<Self, PriceOracleError> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .user_agent("LemiPay/1.0 (lab investment oracle)")
            .build()
            .map_err(|e| PriceOracleError::Fetch(e.to_string()))?;

        let tickers = TickerMap::global();
        let api_key = std::env::var("COINGECKO_API_KEY")
            .ok()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty());
        let key_type = std::env::var("COINGECKO_KEY_TYPE")
            .unwrap_or_else(|_| "demo".into())
            .to_lowercase();

        println!(
            "CoinGecko oracle: {} tickers (hardcoded catalog)",
            tickers.len()
        );
        if api_key.is_some() {
            println!("CoinGecko API key loaded (type={key_type})");
        } else {
            println!("CoinGecko: public API (no key; rate-limited)");
        }

        Ok(Self {
            client,
            cache: PriceCache::new(45),
            mock_fallback: MockPriceOracle::new(),
            tickers,
            api_key,
            key_type,
        })
    }

    async fn fetch_batch(&self, cg_ids: &[String]) -> HashMap<String, BigDecimal> {
        let mut out = HashMap::new();
        if cg_ids.is_empty() {
            return out;
        }
        let base = if self.key_type == "pro" && self.api_key.is_some() {
            "https://pro-api.coingecko.com/api/v3"
        } else {
            "https://api.coingecko.com/api/v3"
        };
        let url = format!(
            "{base}/simple/price?ids={}&vs_currencies=usd",
            cg_ids.join(",")
        );
        let mut req = self.client.get(&url);
        if let Some(key) = &self.api_key {
            let header = if self.key_type == "pro" {
                "x-cg-pro-api-key"
            } else {
                "x-cg-demo-api-key"
            };
            req = req.header(header, key);
        }
        let resp = match req.send().await {
            Ok(r) => r,
            Err(e) => {
                eprintln!("CoinGecko fetch failed: {e}");
                return out;
            }
        };
        if !resp.status().is_success() {
            eprintln!("CoinGecko HTTP {}", resp.status());
            return out;
        }
        let json: serde_json::Value = match resp.json().await {
            Ok(j) => j,
            Err(e) => {
                eprintln!("CoinGecko parse failed: {e}");
                return out;
            }
        };
        for id in cg_ids {
            if let Some(price) = json
                .get(id)
                .and_then(|o| o.get("usd"))
                .and_then(|v| v.as_f64())
            {
                // Reject dust (wrong id often returns ~1e-12 which formats to 0)
                if !price.is_finite() || price < 1e-6 {
                    eprintln!("CoinGecko: ignoring dust/invalid price for id={id} usd={price}");
                    continue;
                }
                if let Some(bd) = f64_to_bd(price) {
                    out.insert(id.clone(), bd);
                }
            } else {
                eprintln!("CoinGecko: no usd price in response for id={id}");
            }
        }
        out
    }
}

/// Parse f64 → BigDecimal without truncating small-but-valid prices to zero.
fn f64_to_bd(price: f64) -> Option<BigDecimal> {
    // Use enough precision; avoid `{:.8}` which turns 2e-12 into "0.00000000"
    BigDecimal::from_str(&format!("{:.12}", price))
        .ok()
        .filter(|d| *d > BigDecimal::from(0))
}

#[async_trait]
impl PriceOracle for CoinGeckoPriceOracle {
    async fn get_usd_prices(
        &self,
        assets: &[AssetPriceRef],
    ) -> Result<HashMap<Uuid, BigDecimal>, PriceOracleError> {
        let mut result = HashMap::new();
        let mut need: Vec<&AssetPriceRef> = Vec::new();

        for a in assets {
            if let Some(p) = self.cache.get(a.id) {
                result.insert(a.id, p);
            } else {
                need.push(a);
            }
        }
        if need.is_empty() {
            return Ok(result);
        }

        // symbol asset_id → cg id
        let mut id_to_assets: HashMap<String, Vec<Uuid>> = HashMap::new();
        let mut unresolved: Vec<&AssetPriceRef> = Vec::new();
        for a in &need {
            match self.tickers.resolve(&a.symbol, &a.external_id) {
                Some(cg_id) => id_to_assets.entry(cg_id).or_default().push(a.id),
                None => {
                    eprintln!(
                        "CoinGecko: no ticker mapping for symbol={} (add it to hardcoded ticker catalog)",
                        a.symbol
                    );
                    unresolved.push(a);
                }
            }
        }

        let cg_ids: Vec<String> = id_to_assets.keys().cloned().collect();
        let prices_by_id = self.fetch_batch(&cg_ids).await;

        for (cg_id, asset_ids) in &id_to_assets {
            if let Some(price) = prices_by_id.get(cg_id) {
                for id in asset_ids {
                    self.cache.put(*id, price.clone());
                    result.insert(*id, price.clone());
                }
            }
        }

        // Fallbacks: stale cache, then mock — always fill so execute never fails on CG gaps
        for a in need {
            if result.contains_key(&a.id) {
                continue;
            }
            if let Some(stale) = self.cache.get_stale(a.id) {
                if stale > BigDecimal::from(0) {
                    eprintln!("CoinGecko: stale cache for {}", a.symbol);
                    result.insert(a.id, stale);
                    continue;
                }
            }
            eprintln!(
                "CoinGecko: mock fallback for {} (missing or invalid API price; check api id in hardcoded ticker catalog)",
                a.symbol
            );
            let mock = self.mock_fallback.prices_sync(std::slice::from_ref(a));
            if let Some(p) = mock.get(&a.id) {
                self.cache.put(a.id, p.clone());
                result.insert(a.id, p.clone());
            }
        }

        // Also mock any unresolved symbols
        for a in unresolved {
            if result.contains_key(&a.id) {
                continue;
            }
            let mock = self.mock_fallback.prices_sync(std::slice::from_ref(a));
            if let Some(p) = mock.get(&a.id) {
                self.cache.put(a.id, p.clone());
                result.insert(a.id, p.clone());
            }
        }

        if result.len() < assets.len() {
            // Last resort: mock anything still missing
            for a in assets {
                if result.contains_key(&a.id) {
                    continue;
                }
                let mock = self.mock_fallback.prices_sync(std::slice::from_ref(a));
                if let Some(p) = mock.get(&a.id) {
                    result.insert(a.id, p.clone());
                }
            }
        }

        if result.len() < assets.len() {
            return Err(PriceOracleError::MissingPrice(
                "could not resolve all asset prices".into(),
            ));
        }
        Ok(result)
    }
}

/// Alias kept so existing imports of CompositePriceOracle still work if any.
pub type CompositePriceOracle = CoinGeckoPriceOracle;

/// Build oracle from env: PRICE_ORACLE_MODE=live|mock
pub fn build_price_oracle() -> Arc<dyn PriceOracle> {
    // Eager-load ticker config so links + prices share the same map.
    let _ = TickerMap::global();

    let mode = std::env::var("PRICE_ORACLE_MODE").unwrap_or_else(|_| "mock".into());
    match mode.to_lowercase().as_str() {
        "live" => match CoinGeckoPriceOracle::new() {
            Ok(o) => {
                println!("Price oracle: live (CoinGecko only)");
                Arc::new(o)
            }
            Err(e) => {
                eprintln!("Failed to build CoinGecko oracle ({e}), falling back to mock");
                Arc::new(MockPriceOracle::new())
            }
        },
        _ => {
            println!("Price oracle: mock");
            Arc::new(MockPriceOracle::new())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn mock_returns_prices_for_symbols() {
        let o = MockPriceOracle::new();
        let id = Uuid::new_v4();
        let assets = vec![AssetPriceRef {
            id,
            symbol: "BTC".into(),
            price_provider: "coingecko".into(),
            external_id: "bitcoin".into(),
        }];
        let prices = o.get_usd_prices(&assets).await.unwrap();
        assert!(prices.contains_key(&id));
        assert!(prices[&id] > BigDecimal::from(0));
    }
}
