CREATE TABLE IF NOT EXISTS friend (
    requester_id UUID NOT NULL REFERENCES "user"(id),
    addressee_id UUID NOT NULL REFERENCES "user"(id),
    status TEXT NOT NULL DEFAULT 'pending',
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (requester_id, addressee_id),
    CHECK (requester_id <> addressee_id)
);
