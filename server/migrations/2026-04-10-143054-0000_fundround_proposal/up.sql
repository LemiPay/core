-- Your SQL goes here
-- =========================
-- FUND ROUND PROPOSAL
-- =========================

CREATE TABLE fund_round_proposal (
    proposal_id UUID PRIMARY KEY,
    
    target_amount NUMERIC NOT NULL,
    currency_id UUID NOT NULL,

    FOREIGN KEY (proposal_id) REFERENCES "proposal"(id) ON DELETE CASCADE,
    FOREIGN KEY (currency_id) REFERENCES "currency"(currency_id) ON DELETE RESTRICT
);