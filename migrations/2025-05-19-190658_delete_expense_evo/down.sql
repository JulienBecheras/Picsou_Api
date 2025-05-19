CREATE TABLE expenses_evo (
    id INT PRIMARY KEY REFERENCES payments(id),
    name VARCHAR NOT NULL,
    share_number INT NOT NULL,
    description TEXT
);