-- Your SQL goes here
CREATE TYPE blockchain AS ENUM (
    'ethereum',
    'sepolia',
    'arbitrum',
    'base',
    'polygon'
    );

CREATE TABLE IF NOT EXISTS currency
(
    currency_id       UUID PRIMARY KEY     DEFAULT gen_random_uuid(),

    name              TEXT        NOT NULL,
    ticker            TEXT        NOT NULL UNIQUE,

    blockchain        blockchain  NOT NULL,
    token_address     TEXT        NOT NULL,
    token_currency_id TEXT,

    decimals          SMALLINT    NOT NULL CHECK (decimals BETWEEN 0 AND 36),
    is_active         BOOLEAN     NOT NULL DEFAULT TRUE,

    created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE (blockchain, token_address)
);

-- Insert initial currency
INSERT INTO currency (currency_id,
                      name,
                      ticker,
                      blockchain,
                      token_address,
                      decimals,
                      is_active)

VALUES ('33de6c7c-62a2-4182-813a-9005183be70d',
        'USD Coin',
        'USDC',
        'sepolia',
        '0x1c7d4b196cb0c7b01d743fbc6116a902379c7238',
        6,
        TRUE)

ON CONFLICT (ticker) DO NOTHING;