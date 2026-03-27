-- This file should undo anything in `up.sql`
-- Dropear tabla
DROP TABLE IF EXISTS user_in_group;

-- Dropear enums
DROP TYPE IF EXISTS group_member_status;
DROP TYPE IF EXISTS group_role;