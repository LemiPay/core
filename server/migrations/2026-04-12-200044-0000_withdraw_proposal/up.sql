-- Your SQL goes here

-- =========================
-- WITHDRAW PROPOSAL
-- =========================

CREATE TABLE withdraw_proposal (
	proposal_id UUID PRIMARY KEY,

	amount NUMERIC NOT NULL CHECK (amount > 0),
    currency_id UUID NOT NULL,

    FOREIGN KEY (proposal_id) REFERENCES "proposal"(id) ON DELETE CASCADE,
    FOREIGN KEY (currency_id) REFERENCES "currency"(currency_id) ON DELETE CASCADE
);

