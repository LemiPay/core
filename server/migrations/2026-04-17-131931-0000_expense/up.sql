-- Your SQL goes here
CREATE TYPE expense_status as ENUM (
    'created',
    'verified',
    'updated',
    'deleted'
);



CREATE TABLE IF NOT EXISTS expense (
    expense_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    currency_id UUID NOT NULL,
    group_id UUID NOT NULL,
    description TEXT,
    amount NUMERIC NOT NULL CHECK (amount > 0),
    status expense_status NOT NULL DEFAULT 'created',
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),


    FOREIGN KEY (user_id) REFERENCES "user"(id) ON DELETE CASCADE,
    FOREIGN KEY (currency_id) REFERENCES currency(currency_id) ON DELETE CASCADE,
    FOREIGN KEY (group_id) REFERENCES "group"(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS expense_participant(
    expense_id UUID      NOT NULL,
    user_id    UUID      NOT NULL,
    amount     NUMERIC   NOT NULL CHECK (amount > 0),
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),


    PRIMARY KEY(expense_id, user_id),

    FOREIGN KEY (expense_id) REFERENCES expense(expense_id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES "user"(id) ON DELETE CASCADE
);