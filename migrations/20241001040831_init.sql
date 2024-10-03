-- Add migration script here

CREATE TABLE IF NOT EXISTS user_detail (
    id SERIAL PRIMARY KEY AUTOINCREMENT,
    name VARCHAR(255) NOT NULL,
    age INT NOT NULL,
    salary DECIMAL(10, 2) NOT NULL,
    created_at TIMESTAMPTZ  DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ  DEFAULT CURRENT_TIMESTAMP
);


INSERT INTO user_detail (name, age, salary) VALUES ('John Doe', 30, 50000.00);
INSERT INTO user_detail (name, age, salary) VALUES ('Jane Doe', 25, 60000.00);
INSERT INTO user_detail (name, age, salary) VALUES ('Jim Beam', 35, 70000.00);
INSERT INTO user_detail (name, age, salary) VALUES ('Jill Hill', 40, 80000.00);
INSERT INTO user_detail (name, age, salary) VALUES ('Jack Daniels', 45, 90000.00);
INSERT INTO user_detail (name, age, salary) VALUES ('Jill Hill', 40, 80000.00);
INSERT INTO user_detail (name, age, salary) VALUES ('Jack Daniels', 45, 90000.00);
INSERT INTO user_detail (name, age, salary) VALUES ('Jill Hill', 40, 80000.00);
INSERT INTO user_detail (name, age, salary) VALUES ('Jack Daniels', 45, 90000.00);