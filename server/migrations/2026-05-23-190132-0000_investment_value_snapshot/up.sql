CREATE TABLE investment_value_snapshot (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    investment_id UUID NOT NULL REFERENCES investment(id) ON DELETE CASCADE,
    value NUMERIC NOT NULL,
    snapshot_date TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_investment_value_snapshot_investment_id ON investment_value_snapshot(investment_id);
