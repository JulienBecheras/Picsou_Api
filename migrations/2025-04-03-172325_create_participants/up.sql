CREATE TABLE participants (
      id SERIAL PRIMARY KEY,
      id_user INT REFERENCES users(id),
      id_payment INT REFERENCES payments(id),
      amount FLOAT NOT NULL,
      created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
      updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TRIGGER set_timestamp_participants
    BEFORE UPDATE ON participants
    FOR EACH ROW
EXECUTE FUNCTION update_timestamp();