--! accounts_count
SELECT COUNT(*)
FROM accounts;

--! get_emission_account
SELECT *
FROM accounts
WHERE accounts.id = 1;

--! get_store_account
SELECT *
FROM accounts
WHERE accounts.id = 2;

--! insert_account
INSERT INTO accounts(username, card_number, password_hash)
VALUES (:username, :card_number, :password_hash);

--! is_account_exists
SELECT is_existing FROM accounts
WHERE card_number = :card_number;

--! mark_account_as_deleted
UPDATE accounts
SET is_existing = FALSE
WHERE card_number = :card_number;

--! get_account
SELECT 
    accounts.username,
    accounts.card_number,
    accounts.is_existing,
    accounts.password_hash
FROM accounts
WHERE card_number = :card_number;

--! get_account_by_token
SELECT 
    a.username,
    a.card_number,
    a.is_existing
FROM tokens
LEFT JOIN accounts a ON tokens.account = a.id
WHERE  tokens.token = :token;

--! get_account_balance
SELECT COALESCE(SUM(recv.amount), 0) - COALESCE(SUM(spnd.amount), 0) AS balance
FROM accounts a
LEFT JOIN transactions recv ON a.id = recv.sender
LEFT JOIN transactions spnd ON a.id = spnd.recipient
WHERE a.card_number = :card_number;

--! list_account_transactions
SELECT 
    t.amount,
    t.created_at,
    sender_account.username AS sender_username,
    sender_account.card_number AS sender_card_number,
    sender_account.is_existing AS sender_is_existing,
    recipient_account.username AS recipient_username,
    recipient_account.card_number AS recipient_card_number,
    recipient_account.is_existing AS recipient_is_existing
FROM transactions t
LEFT JOIN accounts sender_account ON t.sender = sender_account.id
LEFT JOIN accounts recipient_account ON t.recipient = recipient_account.id
WHERE sender_account.card_number = :card_number OR recipient_account.card_number = :card_number;


-- get_accounts
-- SELECT
--     a.username,
--     a.card_number,
--     a.is_existing,
--     COALESCE(SUM(recv.amount), 0) - COALESCE(SUM(spnd.amount), 0) AS balance,
--     ARRAY_AGG(
--         (
--             sender_account.username,
--             sender_account.card_number,
--             sender_account.is_existing,
--             recipient_account.username,
--             recipient_account.card_number,
--             recipient_account.is_existing
--         )
--     ) AS transactions,
--     ARRAY_AGG(t.token) AS tokens
-- FROM accounts a
-- LEFT JOIN transactions recv ON a.id = recv.sender
-- LEFT JOIN transactions spnd ON a.id = spnd.recipient
-- LEFT JOIN accounts sender_account ON recv.sender = sender_account.id
-- LEFT JOIN accounts recipient_account ON spnd.recipient = recipient_account.id
-- LEFT JOIN tokens t ON a.id = t.account
-- GROUP BY a.username, a.card_number, a.is_existing;

--! create_transaction
INSERT INTO transactions(sender, recipient, amount)
VALUES (
    (
        SELECT id FROM accounts WHERE card_number = :sender_card 
    ),
    (
        SELECT id FROM accounts WHERE card_number = :recipient_card
    ),
     :amount
);

--! insert_token
INSERT INTO tokens(account, token)
VALUES (
    (
        SELECT id FROM accounts WHERE card_number = :card_number
    ),
    :token
);
