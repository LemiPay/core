-- Your SQL goes here
CREATE TABLE IF NOT EXISTS group_wallet(
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    address TEXT NOT NULL,
    group_id UUID NOT NULL,
    currency_id UUID NOT NULL,

    balance NUMERIC NOT NULL DEFAULT 0,
    
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW(),

    -- reglas de negocio:
    UNIQUE(group_id, currency_id), -- un grupo no puede tener dos wallets de la misma moneda
    UNIQUE(address, currency_id), -- una dirección no puede repetirse para la misma moneda

    -- foreign keys
    CONSTRAINT fk_group
        FOREIGN KEY (group_id)
        REFERENCES "group"(id)
        ON DELETE CASCADE,

    CONSTRAINT fk_currency
        FOREIGN KEY (currency_id)
        REFERENCES "currency"(currency_id)
        ON DELETE RESTRICT
);