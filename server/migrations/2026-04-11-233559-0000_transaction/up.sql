-- Your SQL goes here
-- =========================
-- ENUM
-- =========================
CREATE TYPE transaction_type AS ENUM (
    'deposit',
    'withdraw',
    'expense',
    'investment'
);

-- El amount esta dupeado asi validamos consistencia en backend.

-- =========================
-- TABLE: transaction
-- =========================
CREATE TABLE IF NOT EXISTS transaction (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    tx_hash TEXT,
    amount NUMERIC NOT NULL CHECK (amount > 0),

    user_id UUID NOT NULL,  -- Author
    group_id UUID NOT NULL,
    currency_id UUID NOT NULL,
    address TEXT NOT NULL,

    description TEXT,

    tx_type transaction_type NOT NULL,

    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    CONSTRAINT fk_transaction_user
        FOREIGN KEY (user_id)
        REFERENCES "user"(id)
        ON DELETE CASCADE,

    CONSTRAINT fk_transaction_group
        FOREIGN KEY (group_id)
        REFERENCES "group"(id)
        ON DELETE CASCADE,

    CONSTRAINT fk_transaction_currency
        FOREIGN KEY (currency_id)
        REFERENCES "currency"(currency_id)
        ON DELETE RESTRICT
);

-- =========================
-- TABLE: transaction_participant
-- =========================
CREATE TABLE IF NOT EXISTS transaction_participant (
    transaction_id UUID NOT NULL,
    user_id UUID NOT NULL,

    amount NUMERIC NOT NULL CHECK (amount > 0),

    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    PRIMARY KEY (transaction_id, user_id),

    CONSTRAINT fk_tp_transaction
        FOREIGN KEY (transaction_id)
        REFERENCES transaction(id)
        ON DELETE CASCADE,

    CONSTRAINT fk_tp_user
        FOREIGN KEY (user_id)
        REFERENCES "user"(id)
        ON DELETE CASCADE
);

-- =========================
-- INDEXES (A futuro pueden estar buenos)
-- =========================
-- CREATE INDEX idx_transaction_user_id ON transaction(user_id);
-- CREATE INDEX idx_transaction_group_id ON transaction(group_id);
-- CREATE INDEX idx_transaction_currency_id ON transaction(currency_id);
-- CREATE INDEX idx_transaction_created_at ON transaction(created_at);

-- CREATE INDEX idx_tp_user_id ON transaction_participant(user_id);
-- CREATE INDEX idx_tp_transaction_id ON transaction_participant(transaction_id);