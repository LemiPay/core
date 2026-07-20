DELETE FROM strategy_allocation
WHERE strategy_id IN (
    'b0000001-0000-4000-8000-000000000001',
    'b0000001-0000-4000-8000-000000000002',
    'b0000001-0000-4000-8000-000000000003',
    'b0000001-0000-4000-8000-000000000004',
    'b0000001-0000-4000-8000-000000000005'
);

-- Restore previous-ish baskets (BIL era) is best-effort; leave strategies names as-is.
UPDATE asset SET is_active = TRUE WHERE symbol = 'BIL';
UPDATE asset SET is_active = FALSE WHERE symbol IN ('SPCX', 'GOLD');
