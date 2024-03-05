CREATE TABLE accounts (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    username VARCHAR(100) NOT NULL,
    card_number VARCHAR(16) NOT NULL,
    password_hash VARCHAR(500) NOT NULL,
    is_existing BOOL NOT NULL
);

CREATE TABLE transactions (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    sender INTEGER NOT NULL REFERENCES accounts(id) ON DELETE RESTRICT,
    recipient INTEGER NOT NULL REFERENCES accounts(id) ON DELETE RESTRICT,
    amount BIGINT NOT NULL
);
 
CREATE TABLE tokens (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    account INTEGER REFERENCES accounts(id) ON DELETE RESTRICT,
    token VARCHAR(30) NOT NULL
);
