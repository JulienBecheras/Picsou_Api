CREATE TABLE refunds (
    id SERIAL PRIMARY KEY,
    amount FLOAT NOT NULL,
    status VARCHAR CHECK (status IN ('pending', 'completed', 'canceled')) NOT NULL,
    contributors_id INT NOT NULL,
    participants_id INT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (participants_id) REFERENCES participants(id),
    FOREIGN KEY (contributors_id) REFERENCES contributors(id)
);

CREATE TRIGGER set_timestamp_refunds
    BEFORE UPDATE ON refunds
    FOR EACH ROW
EXECUTE FUNCTION update_timestamp();