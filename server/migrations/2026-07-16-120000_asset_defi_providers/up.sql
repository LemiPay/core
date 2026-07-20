-- Allow DeFi providers for paper mark-to-market assets
ALTER TABLE asset DROP CONSTRAINT IF EXISTS asset_price_provider_check;
ALTER TABLE asset
    ADD CONSTRAINT asset_price_provider_check
    CHECK (price_provider IN ('coingecko', 'stooq', 'mock', 'dexscreener', 'ondo'));

-- Crypto → DexScreener (on-chain / DeFi price discovery)
UPDATE asset SET
    price_provider = 'dexscreener',
    external_id = symbol
WHERE kind = 'crypto';

-- Equities as tokenized stocks → Ondo (UI link) + DexScreener search id for price
UPDATE asset SET
    price_provider = 'ondo',
    external_id = symbol
WHERE kind = 'tokenized_stock';

-- RWA / T-bill proxy → Ondo treasury narrative + dexscreener-friendly id
UPDATE asset SET
    price_provider = 'ondo',
    external_id = CASE
        WHEN symbol = 'BIL' THEN 'OUSG'
        ELSE symbol
    END
WHERE kind = 'rwa';
