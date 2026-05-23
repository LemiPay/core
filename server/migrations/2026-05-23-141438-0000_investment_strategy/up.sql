CREATE TABLE investment_strategy (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    risk_level TEXT NOT NULL CHECK (risk_level IN ('low', 'medium', 'high')),
    expected_return_percentage NUMERIC NOT NULL CHECK (expected_return_percentage > 0),
    duration_days INTEGER NOT NULL CHECK (duration_days > 0),
    created_at TIMESTAMP NOT NULL DEFAULT NOW()
);

INSERT INTO investment_strategy (name, description, risk_level, expected_return_percentage, duration_days) VALUES
    ('Fondo Común Lemipay', 'Bajo riesgo, retorno estable y predecible', 'low', 3.0, 30),
    ('TOP 100 ARG', 'Riesgo medio, buen retorno balanceado', 'medium', 7.5, 60),
    ('Micheal Saylor', 'Alto riesgo, alto retorno potencial', 'high', 15.0, 90);
