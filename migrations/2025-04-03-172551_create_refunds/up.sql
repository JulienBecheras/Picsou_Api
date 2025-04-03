CREATE TABLE refunds (
        id INT PRIMARY KEY REFERENCES payments(id),
        status TEXT CHECK (status IN ('waiting', 'done', 'cancelled')) NOT NULL
);