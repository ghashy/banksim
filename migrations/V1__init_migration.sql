CREATE TABLE accounts (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    username VARCHAR(100) NOT NULL UNIQUE,
    card_number VARCHAR(16) NOT NULL UNIQUE,
    password_hash VARCHAR(500) NOT NULL,
    is_existing BOOL NOT NULL DEFAULT TRUE
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

-- Check balance before transaction
CREATE OR REPLACE FUNCTION check_balance_before_transaction()
RETURNS TRIGGER AS $$
DECLARE
    balance INTEGER;
    sender_exists BOOLEAN;
    recipient_exists BOOLEAN;
BEGIN
    SELECT EXISTS(SELECT 1 FROM accounts WHERE id = NEW.sender AND is_existing = TRUE) INTO sender_exists;
    SELECT EXISTS(SELECT 1 FROM accounts WHERE id = NEW.recipient AND is_existing = TRUE) INTO recipient_exists;

    IF NOT sender_exists OR NOT recipient_exists THEN
        RAISE EXCEPTION 'Sender or recipient account does not exist or is not active';
    END IF;

    IF NEW.sender = 1 THEN
        RETURN NEW;
    END IF;

    IF NEW.sender = NEW.recipient THEN
        RAISE EXCEPTION 'Sender and recipient cannot be the same';
    END IF;

    IF NEW.amount <= 0 THEN
        RAISE EXCEPTION 'Amount must be greater than 0';
    END IF;

    WITH received_amount AS (
        SELECT recipient, COALESCE(SUM(amount), 0) AS received_total
        FROM transactions
        GROUP BY recipient
    ),
    spent_amount AS (
        SELECT sender, COALESCE(SUM(amount), 0) AS spent_total
        FROM transactions
        GROUP BY sender
    )
    SELECT COALESCE(ra.received_total, 0) - COALESCE(sa.spent_total, 0) INTO balance
    FROM accounts a
    LEFT JOIN received_amount ra ON a.id = ra.recipient
    LEFT JOIN spent_amount sa ON a.id = sa.sender
    WHERE a.id = NEW.sender;

    IF balance < NEW.amount THEN
        RAISE EXCEPTION 'Not enough funds';
    END IF;

    RETURN NEW;
END;
$$ LANGUAGE plpgsql;
CREATE TRIGGER trg_check_balance_before_transaction
BEFORE INSERT OR DELETE ON transactions
FOR EACH ROW EXECUTE FUNCTION check_balance_before_transaction();

