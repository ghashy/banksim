use std::collections::HashMap;
use std::sync::Arc;

use rand::{distributions::Alphanumeric, Rng};
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use rust_decimal::Decimal;
use secrecy::{ExposeSecret, Secret};
use serde::Serialize;
use time::format_description::well_known::iso8601;
use time::format_description::well_known::iso8601::TimePrecision;
use time::format_description::well_known::Iso8601;
use time::OffsetDateTime;
use tokio::sync::watch::{Receiver, Sender};
use tokio::sync::Mutex;
use tokio::sync::MutexGuard;
use tokio::sync::TryLockError;

use crate::domain::card_number::CardNumber;
use crate::{error_chain_fmt, middleware::Credentials};

use banksim_api::init_payment::beneficiaries::Beneficiaries;

const SIMPLE_ISO: Iso8601<6651332276402088934156738804825718784> = Iso8601::<
    {
        iso8601::Config::DEFAULT
            .set_year_is_six_digits(false)
            .set_time_precision(TimePrecision::Second {
                decimal_digits: None,
            })
            .encode()
    },
>;

time::serde::format_description!(iso_format, OffsetDateTime, SIMPLE_ISO);

#[derive(thiserror::Error)]
pub enum BankOperationError {
    #[error("No account")]
    AccountNotFound,
    #[error("No account for token")]
    TokenNotFound,
    #[error("Account was deleted")]
    AccountIsDeleted,
    #[error("Not enough funds for operation")]
    NotEnoughFunds,
    #[error("Account is not authorized")]
    NotAuthorized,
    #[error("Can't perform transaction")]
    BadTransaction,
    #[error("Mutex lock error: {0}")]
    MutexLockError(#[from] TryLockError),
    #[error("Attempt to perform not allowed operation: {0}")]
    BadOperation(String),
}

impl std::fmt::Debug for BankOperationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

#[derive(Clone, Debug)]
pub struct Bank {
    inner: Arc<Mutex<BankInner>>,
}

impl Bank {
    /// Constructor
    pub fn new(cashbox_pass: &Secret<String>, bank_username: &str) -> Self {
        let (tx, _) = tokio::sync::watch::channel(());

        let emission_account = Account {
            card_number: CardNumber::generate(),
            password: cashbox_pass.clone(),
            is_existing: true,
        };

        let store_account = Account {
            card_number: CardNumber::generate(),
            password: cashbox_pass.clone(),
            is_existing: true,
        };

        let bank = BankInner {
            tokens: HashMap::new(),
            accounts: Vec::new(),
            emission_account,
            store_account,
            transactions: Vec::new(),
            bank_username: bank_username.to_string(),
            notifier: tx,
        };
        Bank {
            inner: Arc::new(Mutex::new(bank)),
        }
    }

    /// IMPORTANT: Do not hold that handler across `.await` to avoid deadlock!
    pub async fn handler(&self) -> BankHandler {
        let guard = self.lock().await;
        BankHandler { guard }
    }

    async fn lock(&self) -> MutexGuard<BankInner> {
        self.inner.lock().await
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct Transaction {
    sender: Account,
    recipient: Account,
    amount: i64,
    #[serde(with = "iso_format")]
    datetime: OffsetDateTime,
}

#[derive(Serialize, Clone, Debug)]
pub struct Account {
    card_number: CardNumber,
    #[serde(skip)]
    password: Secret<String>,
    is_existing: bool,
}

impl Account {
    pub fn card(&self) -> CardNumber {
        self.card_number.clone()
    }
}

impl PartialEq for Account {
    fn eq(&self, other: &Self) -> bool {
        self.card_number.eq(&other.card_number)
    }
}

#[derive(Debug)]
struct BankInner {
    tokens: HashMap<String, CardNumber>,
    accounts: Vec<Account>,
    transactions: Vec<Transaction>,
    emission_account: Account,
    store_account: Account,
    bank_username: String,
    notifier: Sender<()>,
}

pub struct BankHandler<'a> {
    guard: MutexGuard<'a, BankInner>,
}

impl<'a> BankHandler<'a> {
    pub fn subscribe(&self) -> Receiver<()> {
        self.guard.notifier.subscribe()
    }

    /// Validate system account credentials
    pub fn authorize_system(
        &self,
        credentials: Credentials,
    ) -> Result<(), BankOperationError> {
        let bank_username = &self.guard.bank_username;
        let password = self.guard.emission_account.password.expose_secret();
        if bank_username.eq(&credentials.username)
            && password.eq(credentials.password.expose_secret())
        {
            Ok(())
        } else {
            Err(BankOperationError::NotAuthorized)
        }
    }

    /// Add a new account
    pub fn add_account(&mut self, password: &Secret<String>) -> CardNumber {
        let account = Account {
            card_number: CardNumber::generate(),
            is_existing: true,
            password: password.clone(),
        };
        self.guard.accounts.push(account.clone());

        self.notify();
        account.card_number
    }

    /// Mark existing account as deleted
    pub fn delete_account(
        &mut self,
        card: CardNumber,
    ) -> Result<(), BankOperationError> {
        let result = match self
            .guard
            .accounts
            .iter_mut()
            .find(|acc| acc.card_number.eq(&card))
        {
            Some(acc) => {
                acc.is_existing = false;
                Ok(())
            }
            None => {
                if self.guard.store_account.card_number.eq(&card) {
                    Err(BankOperationError::BadOperation(
                        "Can't delete store account".to_string(),
                    ))
                } else {
                    Err(BankOperationError::AccountNotFound)
                }
            }
        };

        self.notify();
        result
    }

    /// Get Vec<Account>
    pub fn list_accounts(
        &self,
    ) -> Vec<crate::domain::responses::system_api::Account> {
        let mut accounts = Vec::new();
        for acc in self.guard.accounts.iter() {
            let tokens = self
                .guard
                .tokens
                .iter()
                .filter(|(_, &ref card)| card.eq(&acc.card_number))
                .map(|(&ref token, _)| token.clone())
                .collect();
            accounts.push(crate::domain::responses::system_api::Account {
                card_number: acc.card_number.clone(),
                balance: self.balance(acc),
                transactions: self.account_transactions(acc),
                exists: acc.is_existing,
                tokens,
            })
        }
        accounts
    }

    pub fn authorize_account(
        &self,
        card: &CardNumber,
        password: &Secret<String>,
    ) -> Result<Account, BankOperationError> {
        let account = self.find_account(card)?;
        if !account
            .password
            .expose_secret()
            .eq(password.expose_secret())
        {
            Err(BankOperationError::NotAuthorized)
        } else {
            Ok(account.clone())
        }
    }

    pub fn find_account(
        &self,
        card: &CardNumber,
    ) -> Result<Account, BankOperationError> {
        let account = self
            .guard
            .accounts
            .iter()
            .find(|&acc| acc.card_number.eq(card))
            .or_else(|| {
                if self.guard.store_account.card_number.eq(card) {
                    Some(&self.guard.store_account)
                } else {
                    None
                }
            })
            .ok_or(BankOperationError::AccountNotFound)?;
        if !account.is_existing {
            return Err(BankOperationError::AccountIsDeleted);
        }
        Ok(account.clone())
    }

    pub fn get_store_account(&self) -> Account {
        self.guard.store_account.clone()
    }

    pub fn store_balance(&self) -> i64 {
        let store_acc = &self.guard.store_account;
        self.balance(store_acc)
    }

    pub fn new_transaction(
        &mut self,
        sender: &Account,
        recipient: &Account,
        amount: i64,
    ) -> Result<(), BankOperationError> {
        if sender == recipient {
            return Err(BankOperationError::BadTransaction);
        }

        if self.balance(sender) < amount {
            return Err(BankOperationError::NotEnoughFunds);
        }

        if amount <= 0 {
            return Err(BankOperationError::BadTransaction);
        }

        let transaction = Transaction {
            sender: sender.clone(),
            recipient: recipient.clone(),
            amount,
            datetime: OffsetDateTime::now_utc(),
        };

        self.guard.transactions.push(transaction);

        self.notify();
        Ok(())
    }

    pub fn new_split_transaction(
        &mut self,
        sender: &Account,
        amount: i64,
        beneficiaries: &Beneficiaries,
    ) -> Result<(), BankOperationError> {
        beneficiaries
            .validate()
            .map_err(|_| BankOperationError::BadTransaction)?;
        let mut bfc = Vec::with_capacity(beneficiaries.count());
        for (token, part) in beneficiaries.iter_tokens() {
            let acc = self.get_account_by_token(token)?;
            bfc.push((acc, part));
        }

        if bfc
            .iter()
            .map(|(acc, _)| acc)
            .find(|acc| acc.eq(&sender))
            .is_some()
        {
            return Err(BankOperationError::BadTransaction);
        }

        if self.balance(sender) < amount {
            return Err(BankOperationError::NotEnoughFunds);
        }

        if amount <= 0 {
            return Err(BankOperationError::BadTransaction);
        }

        let amount = Decimal::from_i64(amount).ok_or(
            BankOperationError::BadOperation(
                "Can't convert money correctly".to_string(),
            ),
        )?;

        for (recipient, part) in bfc.into_iter() {
            let amount = (amount * part).round().to_i64().ok_or(
                BankOperationError::BadOperation(
                    "Can't convert money correctly".to_string(),
                ),
            )?;
            let transaction = Transaction {
                sender: sender.clone(),
                recipient: recipient.clone(),
                amount,
                datetime: OffsetDateTime::now_utc(),
            };
            self.guard.transactions.push(transaction);
        }

        self.notify();
        Ok(())
    }

    pub fn open_credit(
        &mut self,
        card: CardNumber,
        amount: i64,
    ) -> Result<(), BankOperationError> {
        let account = self.find_account(&card)?.clone();

        let transaction = Transaction {
            sender: self.guard.emission_account.clone(),
            recipient: account,
            amount,
            datetime: OffsetDateTime::now_utc(),
        };

        self.guard.transactions.push(transaction);

        self.notify();
        Ok(())
    }

    pub fn list_transactions(&self) -> Vec<Transaction> {
        self.guard.transactions.clone()
    }

    pub fn bank_emission(&self) -> i64 {
        self.balance(&self.guard.emission_account)
    }

    pub fn new_card_token(
        &mut self,
        card: CardNumber,
    ) -> Result<String, BankOperationError> {
        let _ = self.find_account(&card)?;
        let token = generate_token();

        self.guard.tokens.insert(token.clone(), card);
        self.notify();
        Ok(token)
    }

    pub fn get_account_by_token(
        &self,
        token: &str,
    ) -> Result<Account, BankOperationError> {
        let card = self
            .guard
            .tokens
            .get(token)
            .ok_or(BankOperationError::TokenNotFound)?
            .clone();
        self.find_account(&card)
    }

    fn balance(&self, account: &Account) -> i64 {
        let balance = self
            .guard
            .transactions
            .iter()
            .filter(|&transaction| {
                transaction.sender.eq(&account)
                    || transaction.recipient.eq(&account)
            })
            .fold(0i64, |amount, transaction| {
                if transaction.sender.eq(&account) {
                    amount - transaction.amount
                } else {
                    amount + transaction.amount
                }
            });
        balance
    }

    fn account_transactions(&self, acc: &Account) -> Vec<Transaction> {
        self.guard
            .transactions
            .iter()
            .filter(|&transaction| {
                transaction.sender.eq(&acc) || transaction.recipient.eq(&acc)
            })
            .cloned()
            .collect()
    }

    /// I want to notify my subscribers to update their accounts info
    /// after every bank lock
    fn notify(&self) {
        if let Err(e) = self.guard.notifier.send(()) {
            tracing::error!("Failed to send bank lock notification: {e}");
        }
    }
}

/// Generate card token.
fn generate_token() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(30)
        .map(char::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rs_merkle::{Hasher, MerkleTree};

    fn make_bank() -> Bank {
        let password = Secret::new(String::from("pass"));
        Bank::new(&password, "test_bank")
    }

    #[tokio::test]
    async fn split_transaction_success() {
        let bank = make_bank();
        let mut handler = bank.handler().await;
        let payer_card = handler.add_account(&Secret::new("pass".to_string()));
        let payer_acc = handler.find_account(&payer_card).unwrap();
        handler.open_credit(payer_card, 500).unwrap();

        let store = handler.get_store_account().card();
        let bfc1 = handler.add_account(&Secret::new("pass".to_string()));
        let bfc2 = handler.add_account(&Secret::new("pass".to_string()));

        let store_tok = handler.new_card_token(store).unwrap();
        let bfc1_tok = handler.new_card_token(bfc1.clone()).unwrap();
        let bfc2_tok = handler.new_card_token(bfc2.clone()).unwrap();

        let bfc =
            Beneficiaries::builder(store_tok, Decimal::from_f32(0.37).unwrap())
                .add(bfc1_tok, Decimal::from_f32(0.31).unwrap())
                .add(bfc2_tok, Decimal::from_f32(0.32).unwrap())
                .build()
                .unwrap();

        handler
            .new_split_transaction(&payer_acc, 256, &bfc)
            .unwrap();

        assert_eq!(handler.balance(&handler.get_store_account()), 95);
        assert_eq!(handler.balance(&handler.find_account(&bfc1).unwrap()), 79);
        assert_eq!(handler.balance(&handler.find_account(&bfc2).unwrap()), 82);
        assert_eq!(handler.balance(&payer_acc), 244);
    }

    #[test]
    #[ignore]
    fn learn_merkle_tree_on_practice() {
        use rs_merkle::algorithms::Sha256;

        let mut tree: MerkleTree<Sha256> = MerkleTree::new();
        let mut leaves =
            vec![Sha256::hash("a".as_bytes()), Sha256::hash("b".as_bytes())];
        tree.append(&mut leaves);
        let root = tree.root().unwrap_or_default();
        println!("No leaves: {}", hex::encode(root));
        tree.commit();
        let root = tree.root().unwrap();
        println!("After commit, a, b leaves: {}", hex::encode(root));

        dbg!(tree.leaves());

        let mut leaves = vec![Sha256::hash("c".as_bytes())];
        tree.append(&mut leaves);
        let root = tree.root().unwrap_or_default();
        println!("Before commit with c: {}", hex::encode(root));
        tree.commit();
        let root = tree.root().unwrap();
        println!("After commit with c: {}", hex::encode(root));
    }
}
