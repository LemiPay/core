DROP TABLE IF EXISTS investment_holding;
DROP TABLE IF EXISTS strategy_allocation;

ALTER TABLE investment
    DROP COLUMN IF EXISTS exit_kind,
    DROP COLUMN IF EXISTS fee_amount;

DELETE FROM investment_strategy
WHERE id IN (
    'b0000001-0000-4000-8000-000000000001',
    'b0000001-0000-4000-8000-000000000002',
    'b0000001-0000-4000-8000-000000000003',
    'b0000001-0000-4000-8000-000000000004',
    'b0000001-0000-4000-8000-000000000005'
);

DROP TABLE IF EXISTS asset;

ALTER TABLE investment_strategy
    DROP COLUMN IF EXISTS valuation_mode,
    DROP COLUMN IF EXISTS category,
    DROP COLUMN IF EXISTS ragequit_fee_bps;

ALTER TABLE investment_strategy
    DROP CONSTRAINT IF EXISTS investment_strategy_expected_return_percentage_check;

ALTER TABLE investment_strategy
    ADD CONSTRAINT investment_strategy_expected_return_percentage_check
        CHECK (expected_return_percentage > 0);
