-- Your SQL goes here
CREATE TABLE IF NOT EXISTS blockchain_sync_state
(
    sync_key             TEXT PRIMARY KEY,
    last_processed_block BIGINT      NOT NULL CHECK (last_processed_block >= 0),
    updated_at           TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Insert interesting first block
INSERT INTO blockchain_sync_state (sync_key,
                                   last_processed_block)

VALUES ('lemipay_vault',
        11023360)
ON CONFLICT (sync_key) DO NOTHING;