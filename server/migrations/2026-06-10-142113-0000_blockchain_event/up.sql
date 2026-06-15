CREATE TABLE IF NOT EXISTS blockchain_event (
    id                UUID        PRIMARY KEY,
    event_type        TEXT        NOT NULL,
    sender            TEXT        NOT NULL,
    wallet_address    TEXT        NOT NULL,
    token_address     TEXT        NOT NULL,
    currency_id       UUID        NOT NULL REFERENCES currency(currency_id),
    gross_amount      NUMERIC     NOT NULL,
    fee_amount        NUMERIC     NOT NULL,
    net_amount        NUMERIC     NOT NULL,
    tx_hash           TEXT        NOT NULL,
    block_number      BIGINT      NOT NULL,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
