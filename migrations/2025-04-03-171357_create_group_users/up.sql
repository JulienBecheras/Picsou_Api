CREATE TABLE groups_users (
    id SERIAL PRIMARY KEY,
    id_user INT NOT NULL,
    id_group INT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TRIGGER set_timestamp_groups_users
    BEFORE UPDATE ON groups_users
    FOR EACH ROW
EXECUTE FUNCTION update_timestamp();