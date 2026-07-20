-- Align catalog with server/config/coingecko_tickers.toml:
-- coins: BTC ETH SOL DOGE
-- stocks: AAPL MSFT NVDA GOOGL SPCX
-- commodities: GOLD
-- Drop BIL / OUSG-style RWA from strategies.

-- ---------------------------------------------------------------------------
-- Clear MTM strategy baskets (rebuild below)
-- ---------------------------------------------------------------------------
DELETE FROM strategy_allocation
WHERE strategy_id IN (
    'b0000001-0000-4000-8000-000000000001',
    'b0000001-0000-4000-8000-000000000002',
    'b0000001-0000-4000-8000-000000000003',
    'b0000001-0000-4000-8000-000000000004',
    'b0000001-0000-4000-8000-000000000005'
);

-- ---------------------------------------------------------------------------
-- Upsert assets (fixed UUIDs)
-- ---------------------------------------------------------------------------
INSERT INTO asset (id, symbol, name, kind, price_provider, external_id, is_active) VALUES
    ('a0000001-0000-4000-8000-000000000001', 'BTC',  'Bitcoin',              'crypto',          'coingecko', 'bitcoin',      TRUE),
    ('a0000001-0000-4000-8000-000000000002', 'ETH',  'Ethereum',             'crypto',          'coingecko', 'ethereum',     TRUE),
    ('a0000001-0000-4000-8000-000000000003', 'SOL',  'Solana',               'crypto',          'coingecko', 'solana',       TRUE),
    ('a0000001-0000-4000-8000-000000000004', 'DOGE', 'Dogecoin',             'crypto',          'coingecko', 'dogecoin',     TRUE),
    ('a0000001-0000-4000-8000-000000000010', 'AAPL', 'Apple',                'tokenized_stock', 'coingecko', 'apple',        TRUE),
    ('a0000001-0000-4000-8000-000000000011', 'MSFT', 'Microsoft',            'tokenized_stock', 'coingecko', 'microsoft',    TRUE),
    ('a0000001-0000-4000-8000-000000000012', 'NVDA', 'NVIDIA',               'tokenized_stock', 'coingecko', 'nvidia',       TRUE),
    ('a0000001-0000-4000-8000-000000000013', 'GOOGL','Alphabet',             'tokenized_stock', 'coingecko', 'alphabet-inc', TRUE),
    ('a0000001-0000-4000-8000-000000000014', 'SPCX', 'SpaceX',               'tokenized_stock', 'coingecko', 'spacex',       TRUE),
    ('a0000001-0000-4000-8000-000000000021', 'GOLD', 'Gold',                 'rwa',             'coingecko', 'gold',         TRUE)
ON CONFLICT (id) DO UPDATE SET
    symbol = EXCLUDED.symbol,
    name = EXCLUDED.name,
    kind = EXCLUDED.kind,
    price_provider = EXCLUDED.price_provider,
    external_id = EXCLUDED.external_id,
    is_active = EXCLUDED.is_active;

-- Deactivate assets no longer in config (e.g. BIL)
UPDATE asset
SET is_active = FALSE
WHERE symbol NOT IN ('BTC', 'ETH', 'SOL', 'DOGE', 'AAPL', 'MSFT', 'NVDA', 'GOOGL', 'SPCX', 'GOLD');

UPDATE asset
SET is_active = TRUE
WHERE symbol IN ('BTC', 'ETH', 'SOL', 'DOGE', 'AAPL', 'MSFT', 'NVDA', 'GOOGL', 'SPCX', 'GOLD');

-- ---------------------------------------------------------------------------
-- Refresh MTM strategies metadata
-- ---------------------------------------------------------------------------
UPDATE investment_strategy SET
    name = 'Crypto Bluechips',
    description = 'Bluechips crypto: BTC, ETH, SOL (precios CoinGecko).',
    risk_level = 'medium',
    expected_return_percentage = 0,
    duration_days = 30,
    valuation_mode = 'mark_to_market',
    category = 'crypto',
    ragequit_fee_bps = 200
WHERE id = 'b0000001-0000-4000-8000-000000000001';

UPDATE investment_strategy SET
    name = 'Crypto Aggressive',
    description = 'Crypto de mayor volatilidad: SOL, ETH, BTC, DOGE.',
    risk_level = 'high',
    expected_return_percentage = 0,
    duration_days = 45,
    valuation_mode = 'mark_to_market',
    category = 'crypto',
    ragequit_fee_bps = 200
WHERE id = 'b0000001-0000-4000-8000-000000000002';

UPDATE investment_strategy SET
    name = 'Tech Equity + SpaceX',
    description = 'Stocks tokenizadas (paper): AAPL, MSFT, NVDA, GOOGL y SPCX (SpaceX). Precios vía CoinGecko.',
    risk_level = 'medium',
    expected_return_percentage = 0,
    duration_days = 60,
    valuation_mode = 'mark_to_market',
    category = 'stocks',
    ragequit_fee_bps = 200
WHERE id = 'b0000001-0000-4000-8000-000000000003';

UPDATE investment_strategy SET
    name = 'Gold Reserve',
    description = 'Exposición 100% a oro (commodities en CoinGecko). Bajo riesgo relativo al equity.',
    risk_level = 'low',
    expected_return_percentage = 0,
    duration_days = 30,
    valuation_mode = 'mark_to_market',
    category = 'rwa',
    ragequit_fee_bps = 100
WHERE id = 'b0000001-0000-4000-8000-000000000004';

UPDATE investment_strategy SET
    name = 'Balanced Mix',
    description = 'Mix crypto + stocks + gold: BTC, ETH, AAPL, SPCX, GOLD.',
    risk_level = 'medium',
    expected_return_percentage = 0,
    duration_days = 45,
    valuation_mode = 'mark_to_market',
    category = 'mixed',
    ragequit_fee_bps = 200
WHERE id = 'b0000001-0000-4000-8000-000000000005';

-- ---------------------------------------------------------------------------
-- Rebuild allocations (weights sum 10000)
-- ---------------------------------------------------------------------------

-- Crypto Bluechips: 50% BTC, 30% ETH, 20% SOL
INSERT INTO strategy_allocation (strategy_id, asset_id, weight_bps) VALUES
    ('b0000001-0000-4000-8000-000000000001', 'a0000001-0000-4000-8000-000000000001', 5000),
    ('b0000001-0000-4000-8000-000000000001', 'a0000001-0000-4000-8000-000000000002', 3000),
    ('b0000001-0000-4000-8000-000000000001', 'a0000001-0000-4000-8000-000000000003', 2000);

-- Crypto Aggressive: 40% SOL, 30% ETH, 20% BTC, 10% DOGE
INSERT INTO strategy_allocation (strategy_id, asset_id, weight_bps) VALUES
    ('b0000001-0000-4000-8000-000000000002', 'a0000001-0000-4000-8000-000000000003', 4000),
    ('b0000001-0000-4000-8000-000000000002', 'a0000001-0000-4000-8000-000000000002', 3000),
    ('b0000001-0000-4000-8000-000000000002', 'a0000001-0000-4000-8000-000000000001', 2000),
    ('b0000001-0000-4000-8000-000000000002', 'a0000001-0000-4000-8000-000000000004', 1000);

-- Tech Equity + SpaceX: 30% AAPL, 25% MSFT, 20% NVDA, 15% GOOGL, 10% SPCX
INSERT INTO strategy_allocation (strategy_id, asset_id, weight_bps) VALUES
    ('b0000001-0000-4000-8000-000000000003', 'a0000001-0000-4000-8000-000000000010', 3000),
    ('b0000001-0000-4000-8000-000000000003', 'a0000001-0000-4000-8000-000000000011', 2500),
    ('b0000001-0000-4000-8000-000000000003', 'a0000001-0000-4000-8000-000000000012', 2000),
    ('b0000001-0000-4000-8000-000000000003', 'a0000001-0000-4000-8000-000000000013', 1500),
    ('b0000001-0000-4000-8000-000000000003', 'a0000001-0000-4000-8000-000000000014', 1000);

-- Gold Reserve: 100% GOLD
INSERT INTO strategy_allocation (strategy_id, asset_id, weight_bps) VALUES
    ('b0000001-0000-4000-8000-000000000004', 'a0000001-0000-4000-8000-000000000021', 10000);

-- Balanced Mix: 30% BTC, 15% ETH, 20% AAPL, 15% SPCX, 20% GOLD
INSERT INTO strategy_allocation (strategy_id, asset_id, weight_bps) VALUES
    ('b0000001-0000-4000-8000-000000000005', 'a0000001-0000-4000-8000-000000000001', 3000),
    ('b0000001-0000-4000-8000-000000000005', 'a0000001-0000-4000-8000-000000000002', 1500),
    ('b0000001-0000-4000-8000-000000000005', 'a0000001-0000-4000-8000-000000000010', 2000),
    ('b0000001-0000-4000-8000-000000000005', 'a0000001-0000-4000-8000-000000000014', 1500),
    ('b0000001-0000-4000-8000-000000000005', 'a0000001-0000-4000-8000-000000000021', 2000);
