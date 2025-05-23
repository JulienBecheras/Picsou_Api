CREATE TABLE expenses (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    montant FLOAT NOT NULL,
    stock_parts INT NOT NULL
);

CREATE TRIGGER set_timestamp_expenses
    BEFORE UPDATE ON expenses
    FOR EACH ROW
EXECUTE FUNCTION update_timestamp();