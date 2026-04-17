-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS expense;
DROP TABLE IF EXISTS expense_participant;


DROP TYPE IF EXISTS expense_status CASCADE;