-- Your SQL goes here
CREATE TYPE group_status as ENUM ('active', 'ended');

CREATE TABLE IF NOT EXISTS "group"(
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    status group_status NOT NULL DEFAULT 'active',
    "created_at" date NOT NULL DEFAULT now(),
    "updated_at" date NOT NULL DEFAULT now()
)
