CREATE TABLE investment_proposal (
    proposal_id UUID PRIMARY KEY REFERENCES proposal(id) ON DELETE CASCADE,
    amount NUMERIC NOT NULL CHECK (amount > 0),
    strategy_id UUID NOT NULL REFERENCES investment_strategy(id) ON DELETE RESTRICT,
    currency_id UUID NOT NULL REFERENCES currency(currency_id) ON DELETE RESTRICT
);
