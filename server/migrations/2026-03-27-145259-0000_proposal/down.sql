-- This file should undo anything in `up.sql`
DROP TABLE IF EXISTS proposal;
DROP TABLE IF EXISTS vote;

DROP TYPE IF EXISTS proposal_status;
DROP TYPE IF EXISTS vote_type;