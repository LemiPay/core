-- Your SQL goes here
CREATE TABLE IF NOT EXISTS "user" (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    password TEXT NOT NULL,
    name TEXT,
    email TEXT
);