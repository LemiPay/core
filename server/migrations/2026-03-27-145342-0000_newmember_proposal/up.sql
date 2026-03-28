-- Your SQL goes here
-- =========================
-- NEW MEMBER PROPOSAL
-- =========================

CREATE TABLE new_member_proposal (
    proposal_id UUID PRIMARY KEY,

    new_member_id UUID NOT NULL,

    FOREIGN KEY (proposal_id) REFERENCES proposal(id) ON DELETE CASCADE,
    FOREIGN KEY (new_member_id) REFERENCES "user"(id) ON DELETE CASCADE
);