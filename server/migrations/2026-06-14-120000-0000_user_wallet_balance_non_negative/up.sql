UPDATE user_wallet SET balance = 0 WHERE balance < 0;

ALTER TABLE user_wallet
    ADD CONSTRAINT user_wallet_balance_non_negative CHECK (balance >= 0);