CREATE TABLE temperature (
    id SERIAL PRIMARY KEY,
    degrees FLOAT NOT NULL
);

INSERT INTO temperature(degrees) VALUES (25);