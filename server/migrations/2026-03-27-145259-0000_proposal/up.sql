-- Your SQL goes here
-- =========================
-- ENUMS
-- =========================

CREATE TYPE proposal_status AS ENUM (
    'pending',
    'approved',
    'rejected',
    'executed'
);

CREATE TYPE vote_type AS ENUM (
    'yes',
    'no',
    'abstain'
);

-- =========================
-- BASE TABLE: PROPOSAL
-- =========================

CREATE TABLE proposal (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    group_id UUID NOT NULL,
    created_by UUID NOT NULL,

    status proposal_status NOT NULL DEFAULT 'pending',

    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    FOREIGN KEY (group_id) REFERENCES "group"(id) ON DELETE CASCADE,
    FOREIGN KEY (created_by) REFERENCES "user"(id) ON DELETE CASCADE
);

-- =========================
-- VOTES
-- =========================

CREATE TABLE vote (
    proposal_id UUID NOT NULL,
    user_id UUID NOT NULL,

    vote vote_type NOT NULL,

    created_at TIMESTAMP NOT NULL DEFAULT NOW(),

    PRIMARY KEY (proposal_id, user_id),

    FOREIGN KEY (proposal_id) REFERENCES proposal(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES "user"(id) ON DELETE CASCADE
);