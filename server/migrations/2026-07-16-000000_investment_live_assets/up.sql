-- Market instruments catalog
CREATE TABLE asset (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    symbol TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    kind TEXT NOT NULL CHECK (kind IN ('crypto', 'tokenized_stock', 'rwa')),
    price_provider TEXT NOT NULL CHECK (price_provider IN ('coingecko', 'stooq', 'mock')),
    external_id TEXT NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_asset_provider ON asset (price_provider);

-- Strategy valuation mode + category + ragequit fee
ALTER TABLE investment_strategy
    DROP CONSTRAINT IF EXISTS investment_strategy_expected_return_percentage_check;

ALTER TABLE investment_strategy
    ADD COLUMN valuation_mode TEXT NOT NULL DEFAULT 'simulated'
        CHECK (valuation_mode IN ('simulated', 'mark_to_market')),
    ADD COLUMN category TEXT NOT NULL DEFAULT 'simulated'
        CHECK (category IN ('simulated', 'crypto', 'stocks', 'mixed', 'rwa')),
    ADD COLUMN ragequit_fee_bps INTEGER NOT NULL DEFAULT 200
        CHECK (ragequit_fee_bps >= 0 AND ragequit_fee_bps <= 10000);

ALTER TABLE investment_strategy
    ADD CONSTRAINT investment_strategy_expected_return_percentage_check
        CHECK (expected_return_percentage >= 0);

UPDATE investment_strategy
SET valuation_mode = 'simulated', category = 'simulated'
WHERE valuation_mode = 'simulated';

-- Strategy basket allocations (weights in basis points, 10000 = 100%)
CREATE TABLE strategy_allocation (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    strategy_id UUID NOT NULL REFERENCES investment_strategy(id) ON DELETE CASCADE,
    asset_id UUID NOT NULL REFERENCES asset(id) ON DELETE RESTRICT,
    weight_bps INTEGER NOT NULL CHECK (weight_bps > 0 AND weight_bps <= 10000),
    UNIQUE (strategy_id, asset_id)
);

CREATE INDEX idx_strategy_allocation_strategy ON strategy_allocation (strategy_id);

-- Per-investment holdings (units bought at execute for MTM strategies)
CREATE TABLE investment_holding (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    investment_id UUID NOT NULL REFERENCES investment(id) ON DELETE CASCADE,
    asset_id UUID NOT NULL REFERENCES asset(id) ON DELETE RESTRICT,
    units NUMERIC NOT NULL CHECK (units > 0),
    weight_bps_at_entry INTEGER NOT NULL CHECK (weight_bps_at_entry > 0),
    cost_basis_usd NUMERIC NOT NULL CHECK (cost_basis_usd >= 0),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    UNIQUE (investment_id, asset_id)
);

CREATE INDEX idx_investment_holding_investment ON investment_holding (investment_id);

-- Exit metadata on investment
ALTER TABLE investment
    ADD COLUMN exit_kind TEXT
        CHECK (exit_kind IS NULL OR exit_kind IN ('maturity', 'ragequit')),
    ADD COLUMN fee_amount NUMERIC;

-- ---------------------------------------------------------------------------
-- Seed assets (fixed UUIDs for stable allocations)
-- ---------------------------------------------------------------------------
INSERT INTO asset (id, symbol, name, kind, price_provider, external_id) VALUES
    ('a0000001-0000-4000-8000-000000000001', 'BTC',  'Bitcoin',              'crypto',          'coingecko', 'bitcoin'),
    ('a0000001-0000-4000-8000-000000000002', 'ETH',  'Ethereum',             'crypto',          'coingecko', 'ethereum'),
    ('a0000001-0000-4000-8000-000000000003', 'SOL',  'Solana',               'crypto',          'coingecko', 'solana'),
    ('a0000001-0000-4000-8000-000000000004', 'DOGE', 'Dogecoin',             'crypto',          'coingecko', 'dogecoin'),
    ('a0000001-0000-4000-8000-000000000010', 'AAPL', 'Apple (Ondo proxy)',   'tokenized_stock', 'stooq',     'aapl.us'),
    ('a0000001-0000-4000-8000-000000000011', 'MSFT', 'Microsoft (Ondo proxy)','tokenized_stock','stooq',     'msft.us'),
    ('a0000001-0000-4000-8000-000000000012', 'NVDA', 'NVIDIA (Ondo proxy)',  'tokenized_stock', 'stooq',     'nvda.us'),
    ('a0000001-0000-4000-8000-000000000013', 'GOOGL','Alphabet (Ondo proxy)','tokenized_stock', 'stooq',     'googl.us'),
    ('a0000001-0000-4000-8000-000000000020', 'BIL',  'T-Bill ETF (OUSG proxy)','rwa',           'stooq',     'bil.us');

-- ---------------------------------------------------------------------------
-- Seed mark-to-market strategies
-- ---------------------------------------------------------------------------
INSERT INTO investment_strategy (
    id, name, description, risk_level, expected_return_percentage, duration_days,
    valuation_mode, category, ragequit_fee_bps
) VALUES
    (
        'b0000001-0000-4000-8000-000000000001',
        'Crypto Bluechips',
        'Portfolio paper de bluechips crypto (BTC, ETH, SOL) con precios de mercado en vivo.',
        'medium', 0, 30, 'mark_to_market', 'crypto', 200
    ),
    (
        'b0000001-0000-4000-8000-000000000002',
        'Crypto Aggressive',
        'Portfolio crypto de mayor volatilidad orientado a crecimiento (SOL, ETH, BTC, DOGE).',
        'high', 0, 45, 'mark_to_market', 'crypto', 200
    ),
    (
        'b0000001-0000-4000-8000-000000000003',
        'Ondo Tech Equity',
        'Exposición paper a big-tech estilo stocks tokenizadas Ondo (proxies AAPL/MSFT/NVDA/GOOGL). No adquiere tokens Ondo on-chain.',
        'medium', 0, 60, 'mark_to_market', 'stocks', 200
    ),
    (
        'b0000001-0000-4000-8000-000000000004',
        'Ondo Treasury',
        'Proxy de T-bills / OUSG (ETF BIL) para rendimiento estable de bajo riesgo. Portfolio paper.',
        'low', 0, 30, 'mark_to_market', 'rwa', 100
    ),
    (
        'b0000001-0000-4000-8000-000000000005',
        'Balanced Mix',
        'Mix crypto + equities tokenizadas + T-bill para diversificar riesgo de mercado.',
        'medium', 0, 45, 'mark_to_market', 'mixed', 200
    );

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

-- Ondo Tech Equity: 40% AAPL, 30% MSFT, 20% NVDA, 10% GOOGL
INSERT INTO strategy_allocation (strategy_id, asset_id, weight_bps) VALUES
    ('b0000001-0000-4000-8000-000000000003', 'a0000001-0000-4000-8000-000000000010', 4000),
    ('b0000001-0000-4000-8000-000000000003', 'a0000001-0000-4000-8000-000000000011', 3000),
    ('b0000001-0000-4000-8000-000000000003', 'a0000001-0000-4000-8000-000000000012', 2000),
    ('b0000001-0000-4000-8000-000000000003', 'a0000001-0000-4000-8000-000000000013', 1000);

-- Ondo Treasury: 100% BIL
INSERT INTO strategy_allocation (strategy_id, asset_id, weight_bps) VALUES
    ('b0000001-0000-4000-8000-000000000004', 'a0000001-0000-4000-8000-000000000020', 10000);

-- Balanced Mix: 35% BTC, 15% ETH, 25% AAPL, 25% BIL
INSERT INTO strategy_allocation (strategy_id, asset_id, weight_bps) VALUES
    ('b0000001-0000-4000-8000-000000000005', 'a0000001-0000-4000-8000-000000000001', 3500),
    ('b0000001-0000-4000-8000-000000000005', 'a0000001-0000-4000-8000-000000000002', 1500),
    ('b0000001-0000-4000-8000-000000000005', 'a0000001-0000-4000-8000-000000000010', 2500),
    ('b0000001-0000-4000-8000-000000000005', 'a0000001-0000-4000-8000-000000000020', 2500);
