-- Cannot easily remove enum value in Postgres; leave 'liquidated' in place.
ALTER TABLE investment DROP CONSTRAINT IF EXISTS investment_exit_kind_check;
ALTER TABLE investment
    ADD CONSTRAINT investment_exit_kind_check
    CHECK (exit_kind IS NULL OR exit_kind IN ('maturity', 'ragequit'));

ALTER TABLE investment DROP COLUMN IF EXISTS entry_exposure;

ALTER TABLE investment_strategy DROP COLUMN IF EXISTS leverage;
