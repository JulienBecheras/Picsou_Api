CREATE TABLE friend_requests (
                         id SERIAL PRIMARY KEY,
                         from_user_id INT NOT NULL,
                         to_user_id INT NOT NULL,
                         created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
                         updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
                         FOREIGN KEY (from_user_id) REFERENCES users(id) ON DELETE CASCADE,
                         FOREIGN KEY (to_user_id) REFERENCES users(id)ON DELETE CASCADE
);

CREATE TRIGGER set_timestamp_friend_requests
    BEFORE UPDATE ON friend_requests
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();
