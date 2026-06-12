ALTER TYPE transaction_type RENAME TO transaction_type_old;

CREATE TYPE transaction_type AS ENUM (
    'deposit',
    'withdraw',
    'expense',
    'investment'
);

ALTER TABLE transaction
    ALTER COLUMN tx_type TYPE transaction_type
    USING tx_type::text::transaction_type;

DROP TYPE transaction_type_old;
