-- This file should undo anything in `up.sql`
-- =========================
-- DROP INDEXES
-- =========================
-- DROP INDEX IF EXISTS idx_tp_transaction_id;
-- DROP INDEX IF EXISTS idx_tp_user_id;

-- DROP INDEX IF EXISTS idx_transaction_created_at;
-- DROP INDEX IF EXISTS idx_transaction_currency_id;
-- DROP INDEX IF EXISTS idx_transaction_group_id;
-- DROP INDEX IF EXISTS idx_transaction_user_id;

-- =========================
-- DROP TABLES
-- =========================
DROP TABLE IF EXISTS transaction_participant;
DROP TABLE IF EXISTS transaction;

-- =========================
-- DROP ENUM
-- =========================
DROP TYPE IF EXISTS transaction_type;