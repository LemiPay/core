CREATE TABLE investment_member (
       id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
       investment_id UUID NOT NULL REFERENCES investment(id) ON DELETE CASCADE,
       user_id UUID NOT NULL REFERENCES "user"(id) ON DELETE RESTRICT,

       balance_at_investment NUMERIC NOT NULL,

       participation_pct NUMERIC NOT NULL CHECK (participation_pct > 0 AND participation_pct <= 100),

       invested_amount NUMERIC NOT NULL CHECK (invested_amount > 0),

       returned_amount NUMERIC,
       withdrawn_at TIMESTAMP,

       created_at TIMESTAMP NOT NULL DEFAULT NOW(),

       UNIQUE (investment_id, user_id)
);

CREATE INDEX idx_investment_member_investment_id ON investment_member(investment_id);
CREATE INDEX idx_investment_member_user_id ON investment_member(user_id);
