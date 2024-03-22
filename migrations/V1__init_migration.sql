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
BEGIN
        IF NEW.sender = 1 THEN
            RETURN NEW;
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
        SELECT ra.received_total - sa.spent_total INTO balance
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

