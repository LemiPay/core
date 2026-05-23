CREATE TYPE investment_status AS ENUM ('active', 'matured', 'withdrawn');

CREATE TABLE investment (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    proposal_id UUID NOT NULL REFERENCES proposal(id) ON DELETE RESTRICT,
    amount NUMERIC NOT NULL CHECK (amount > 0),
    expected_return NUMERIC NOT NULL CHECK (expected_return >= 0),
    actual_return NUMERIC,
    status investment_status NOT NULL DEFAULT 'active',
    started_at TIMESTAMP NOT NULL DEFAULT NOW(),
    matures_at TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
