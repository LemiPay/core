-- Your SQL goes here
CREATE TABLE IF NOT EXISTS user_wallet(
    address TEXT PRIMARY KEY,
    user_id UUID NOT NULL,
    balance NUMERIC NOT NULL DEFAULT 0,
    currency_id UUID NOT NULL,

    FOREIGN KEY (user_id) REFERENCES "user"(id) ON DELETE CASCADE,
    FOREIGN KEY (currency_id) REFERENCES currency(currency_id) ON DELETE RESTRICT
);