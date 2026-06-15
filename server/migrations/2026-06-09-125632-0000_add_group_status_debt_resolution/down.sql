-- This file should undo anything in `up.sql`
ALTER TYPE group_status RENAME TO group_status_old;
CREATE TYPE group_status AS ENUM ('active', 'ended');
ALTER TABLE "group" ALTER COLUMN status TYPE group_status USING status::text::group_status;
DROP TYPE group_status_old;