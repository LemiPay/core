-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS vote;
DROP TABLE IF EXISTS proposal;

DROP TYPE IF EXISTS proposal_status CASCADE;
DROP TYPE IF EXISTS vote_type CASCADE;