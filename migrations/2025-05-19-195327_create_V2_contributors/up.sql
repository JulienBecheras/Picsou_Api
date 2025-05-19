CREATE TABLE contributors (
    id SERIAL PRIMARY KEY,
    amount_contributed FLOAT NOT NULL,
    groups_users_id INT NOT NULL,
    expenses_id INT NOT NULL,
    FOREIGN KEY (groups_users_id) REFERENCES groups_users(id),
    FOREIGN KEY (expenses_id) REFERENCES expenses(id)
);