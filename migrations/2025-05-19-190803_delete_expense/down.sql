CREATE TABLE expenses (
    id INT PRIMARY KEY REFERENCES payments(id),
    name VARCHAR NOT NULL,
    description TEXT
);