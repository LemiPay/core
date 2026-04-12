-- Your SQL goes here

-- =========================
-- WITHDRAW PROPOSAL
-- =========================

CREATE TABLE withdraw_proposal (
	proposal_id UUID PRIMARY KEY,

	amount NUMERIC NOT NULL CHECK (amount > 0),

	FOREIGN KEY (proposal_id) REFERENCES "proposal"(id) ON DELETE CASCADE
);

