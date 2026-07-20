UPDATE asset SET
    price_provider = 'coingecko',
    external_id = CASE symbol
        WHEN 'BTC' THEN 'bitcoin'
        WHEN 'ETH' THEN 'ethereum'
        WHEN 'SOL' THEN 'solana'
        WHEN 'DOGE' THEN 'dogecoin'
        ELSE lower(symbol)
    END
WHERE kind = 'crypto';

UPDATE asset SET
    price_provider = 'stooq',
    external_id = lower(symbol) || '.us'
WHERE kind = 'tokenized_stock';

UPDATE asset SET
    price_provider = 'stooq',
    external_id = 'bil.us'
WHERE kind = 'rwa';

ALTER TABLE asset DROP CONSTRAINT IF EXISTS asset_price_provider_check;
ALTER TABLE asset
    ADD CONSTRAINT asset_price_provider_check
    CHECK (price_provider IN ('coingecko', 'stooq', 'mock'));
