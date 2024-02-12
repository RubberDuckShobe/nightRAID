-- Add migration script here
CREATE TABLE
    users (
        id SERIAL PRIMARY KEY,
        username TEXT NOT NULL,
        login_token TEXT NOT NULL,
        balance INTEGER NOT NULL,
        machine_address TEXT NOT NULL,
        last_login TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
    );