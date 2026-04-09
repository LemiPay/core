-- Your SQL goes here
CREATE TABLE currency(
    currency_id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    ticker TEXT UNIQUE NOT NULL
);
INSERT INTO currency (currency_id, name, ticker)
VALUES (
           '33de6c7c-62a2-4182-813a-9005183be70d',
           'USD Coin',
           'USDC'
       );