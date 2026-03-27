-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "group"(
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    description TEXT NOT NULL
)
