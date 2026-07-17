-- Leverage on strategies + liquidation status on investments

ALTER TABLE investment_strategy
    ADD COLUMN IF NOT EXISTS leverage INTEGER NOT NULL DEFAULT 1
        CHECK (leverage >= 1);

-- Entry exposure E0 = margin * leverage (units sized to this notional)
ALTER TABLE investment
    ADD COLUMN IF NOT EXISTS entry_exposure NUMERIC;

UPDATE investment
SET entry_exposure = amount
WHERE entry_exposure IS NULL;

ALTER TABLE investment
    ALTER COLUMN entry_exposure SET NOT NULL;

-- Allow liquidation exit kind
ALTER TABLE investment DROP CONSTRAINT IF EXISTS investment_exit_kind_check;
ALTER TABLE investment
    ADD CONSTRAINT investment_exit_kind_check
    CHECK (exit_kind IS NULL OR exit_kind IN ('maturity', 'ragequit', 'liquidation'));

-- New terminal status: liquidated (margin burned)
ALTER TYPE investment_status ADD VALUE IF NOT EXISTS 'liquidated';
