-- Your SQL goes here
CREATE TABLE IF NOT EXISTS fund_round_contribution (
    fund_round_proposal_id UUID NOT NULL,
    user_id UUID NOT NULL,

    amount NUMERIC NOT NULL CHECK (amount > 0),
    transaction_id UUID NOT NULL,

    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    PRIMARY KEY (fund_round_proposal_id, user_id),

    -- FKs
    CONSTRAINT fk_frc_proposal
        FOREIGN KEY (fund_round_proposal_id)
        REFERENCES fund_round_proposal(proposal_id)
        ON DELETE CASCADE,

    CONSTRAINT fk_frc_user
        FOREIGN KEY (user_id)
        REFERENCES "user"(id)
        ON DELETE CASCADE,

    CONSTRAINT fk_frc_transaction
        FOREIGN KEY (transaction_id)
        REFERENCES transaction(id)
        ON DELETE CASCADE
);