-- Your SQL goes here
CREATE TABLE currency(
    currency_id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    ticker TEXT UNIQUE NOT NULL
);
INSERT INTO currency (currency_id, name, ticker)
VALUES (
           gen_random_uuid(),
           'USD Coin',
           'USDC'
       );