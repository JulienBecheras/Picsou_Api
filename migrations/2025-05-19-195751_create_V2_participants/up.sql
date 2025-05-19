CREATE TABLE participants (
    id SERIAL PRIMARY KEY,
    amount_participated FLOAT NOT NULL,
    part_number INT,
    expenses_id INT NOT NULL,
    groups_users_id INT NOT NULL,
    FOREIGN KEY (groups_users_id) REFERENCES groups_users(id) ON DELETE CASCADE,
    FOREIGN KEY (expenses_id) REFERENCES expenses(id) ON DELETE CASCADE
);