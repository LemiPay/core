-- Your SQL goes here
CREATE TABLE IF NOT EXISTS user_wallet(
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    address TEXT NOT NULL,
    user_id UUID NOT NULL,
    currency_id UUID NOT NULL,

    balance NUMERIC NOT NULL DEFAULT 0,
    
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    --
    UNIQUE(address, currency_id),  -- una dirección no puede tener dos wallets de la misma moneda
    
    -- foreign keys
    CONSTRAINT fk_user
        FOREIGN KEY (user_id)
        REFERENCES "user"(id)
        ON DELETE CASCADE,

    CONSTRAINT fk_currency
        FOREIGN KEY (currency_id)
        REFERENCES "currency"(currency_id)
        ON DELETE RESTRICT
);