-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS expense_participant;
DROP TABLE IF EXISTS expense;


DROP TYPE IF EXISTS expense_status CASCADE;