use std::collections::HashMap;
use std::sync::Arc;

use axum::async_trait;
use banksim_api::init_payment::beneficiaries::Beneficiaries;
use rust_decimal::prelude::{FromPrimitive, ToPrimitive};
use rust_decimal::Decimal;
use secrecy::{ExposeSecret, Secret};
use time::OffsetDateTime;
use tokio::sync::watch::{Receiver, Sender};
use tokio::sync::{Mutex, MutexGuard};

use crate::domain::card_number::CardNumber;
use crate::middleware::Credentials;
use crate::Settings;

use super::backend::{BankDataBackend, InitBankDataBackend};
use super::{generate_token, Account, BankOperationError, Transaction};

#[derive(Debug)]
pub struct MemoryStorage(Mutex<Inner>);

#[derive(Debug)]
pub struct Inner {
    tokens: HashMap<String, CardNumber>,
    accounts: Vec<Account>,
    transactions: Vec<Transaction>,
    // System account
    emission_account: Account,
    // We have only single store currently
    store_account: Account,
    notifier: Sender<()>,
}

impl MemoryStorage {
    async fn lock(&self) -> MutexGuard<Inner> {
        self.0.lock().await
    }

    fn balance(&self, guard: &MutexGuard<Inner>, account: &Account) -> i64 {
        let balance = guard
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

    fn account_transactions(
        &self,
        guard: &MutexGuard<Inner>,
        acc: &Account,
    ) -> Vec<Transaction> {
        guard
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
    fn notify(&self, guard: &MutexGuard<Inner>) {
        if let Err(e) = guard.notifier.send(()) {
            tracing::error!("Failed to send bank lock notification: {e}");
        }
    }

    fn find_account(
        &self,
        guard: &MutexGuard<Inner>,
        card: &CardNumber,
    ) -> Result<Account, BankOperationError> {
        let account = guard
            .accounts
            .iter()
            .find(|&acc| acc.card_number.eq(card))
            .or_else(|| {
                if guard.store_account.card_number.eq(card) {
                    Some(&guard.store_account)
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

    fn get_account_by_token(
        &self,
        guard: &MutexGuard<Inner>,
        token: &str,
    ) -> Result<Account, BankOperationError> {
        let card = guard
            .tokens
            .get(token)
            .ok_or(BankOperationError::TokenNotFound)?
            .clone();
        self.find_account(guard, &card)
    }
}

impl InitBankDataBackend for MemoryStorage {
    fn new(
        settings: &Settings,
        tx: Sender<()>,
    ) -> Arc<dyn BankDataBackend + Send + Sync> {
        let emission_account = Account {
            card_number: CardNumber::generate(),
            password: settings.terminal_settings.password.clone(),
            is_existing: true,
            username: settings.bank_username.clone(),
        };

        let store_account = Account {
            card_number: CardNumber::generate(),
            password: settings.terminal_settings.password.clone(),
            is_existing: true,
            username: "store".to_string(),
        };

        Arc::new(MemoryStorage(Mutex::new(Inner {
            tokens: HashMap::new(),
            accounts: Vec::new(),
            emission_account,
            store_account,
            transactions: Vec::new(),
            notifier: tx,
        })))
    }
}

#[async_trait]
impl BankDataBackend for MemoryStorage {
    async fn subscribe(&self) -> Receiver<()> {
        let guard = self.lock().await;

        guard.notifier.subscribe()
    }

    /// Validate system account credentials
    async fn authorize_system(
        &self,
        credentials: Credentials,
    ) -> Result<(), BankOperationError> {
        let guard = self.lock().await;

        let bank_username = &guard.emission_account.username;
        let password = guard.emission_account.password.expose_secret();
        if bank_username.eq(&credentials.username)
            && password.eq(credentials.password.expose_secret())
        {
            Ok(())
        } else {
            Err(BankOperationError::NotAuthorized)
        }
    }

    /// Add a new account
    async fn add_account(
        &self,
        username: &str,
        password: &Secret<String>,
    ) -> Result<CardNumber, BankOperationError> {
        let mut guard = self.lock().await;

        let account = Account {
            card_number: CardNumber::generate(),
            is_existing: true,
            password: password.clone(),
            username: username.to_string(),
        };
        guard.accounts.push(account.clone());

        self.notify(&guard);
        Ok(account.card_number)
    }

    /// Mark existing account as deleted
    async fn delete_account(
        &self,
        card: &CardNumber,
    ) -> Result<(), BankOperationError> {
        let mut guard = self.lock().await;

        let result = match guard
            .accounts
            .iter_mut()
            .find(|acc| acc.card_number.eq(&card))
        {
            Some(acc) => {
                if acc.is_existing {
                    acc.is_existing = false;
                    Ok(())
                } else {
                    Err(BankOperationError::AccountIsDeleted)
                }
            }
            None => {
                if guard.store_account.card_number.eq(&card) {
                    Err(BankOperationError::BadOperation(
                        "Can't delete store account".to_string(),
                    ))
                } else {
                    Err(BankOperationError::AccountNotFound)
                }
            }
        };

        self.notify(&guard);
        result
    }

    /// Get Vec<Account>
    async fn list_accounts(
        &self,
    ) -> Result<
        Vec<crate::domain::responses::system_api::Account>,
        BankOperationError,
    > {
        let guard = self.lock().await;

        let mut accounts = Vec::new();
        for acc in guard.accounts.iter() {
            let tokens = guard
                .tokens
                .iter()
                .filter(|(_, &ref card)| card.eq(&acc.card_number))
                .map(|(&ref token, _)| token.clone())
                .collect();
            accounts.push(crate::domain::responses::system_api::Account {
                card_number: acc.card_number.clone(),
                balance: self.balance(&guard, acc),
                transactions: self.account_transactions(&guard, acc),
                exists: acc.is_existing,
                tokens,
                username: acc.username.clone(),
            })
        }
        Ok(accounts)
    }

    async fn authorize_account(
        &self,
        card: &CardNumber,
        password: &Secret<String>,
    ) -> Result<Account, BankOperationError> {
        let guard = self.lock().await;

        let account = self.find_account(&guard, card)?;
        if !account
            .password
            .expose_secret()
            .eq(password.expose_secret())
        {
            eprintln!(
                "given: {}, need: {}",
                password.expose_secret(),
                account.password.expose_secret()
            );
            Err(BankOperationError::NotAuthorized)
        } else {
            Ok(account.clone())
        }
    }

    async fn find_account(
        &self,
        card: &CardNumber,
    ) -> Result<Account, BankOperationError> {
        let guard = self.lock().await;

        self.find_account(&guard, card)
    }

    async fn get_store_account(&self) -> Result<Account, BankOperationError> {
        let guard = self.lock().await;

        Ok(guard.store_account.clone())
    }

    async fn store_balance(&self) -> Result<i64, BankOperationError> {
        let guard = self.lock().await;

        let store_acc = &guard.store_account;
        Ok(self.balance(&guard, store_acc))
    }

    async fn balance(
        &self,
        card: &CardNumber,
    ) -> Result<i64, BankOperationError> {
        let guard = self.lock().await;

        let acc = self.find_account(&guard, card)?;
        Ok(self.balance(&guard, &acc))
    }

    async fn new_transaction(
        &self,
        sender: &CardNumber,
        recipient: &CardNumber,
        amount: i64,
    ) -> Result<(), BankOperationError> {
        let mut guard = self.lock().await;

        let sender = self.find_account(&guard, sender)?;
        let recipient = self.find_account(&guard, recipient)?;

        if sender == recipient {
            return Err(BankOperationError::BadTransaction);
        }

        if self.balance(&guard, &sender) < amount {
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

        guard.transactions.push(transaction);

        self.notify(&guard);
        Ok(())
    }

    async fn new_split_transaction(
        &self,
        sender: &CardNumber,
        amount: i64,
        beneficiaries: &Beneficiaries,
    ) -> Result<(), BankOperationError> {
        let mut guard = self.lock().await;

        beneficiaries
            .validate()
            .map_err(|_| BankOperationError::BadTransaction)?;
        let mut bfc = Vec::with_capacity(beneficiaries.count());
        for (token, part) in beneficiaries.iter_tokens() {
            let acc = self.get_account_by_token(&guard, token)?;
            bfc.push((acc, part));
        }

        if bfc
            .iter()
            .map(|(acc, _)| acc)
            .find(|acc| acc.card_number.eq(&sender))
            .is_some()
        {
            return Err(BankOperationError::BadTransaction);
        }

        let sender = self.find_account(&guard, sender)?;
        if self.balance(&guard, &sender) < amount {
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
            guard.transactions.push(transaction);
        }

        self.notify(&guard);
        Ok(())
    }

    async fn open_credit(
        &self,
        card: &CardNumber,
        amount: i64,
    ) -> Result<(), BankOperationError> {
        let mut guard = self.lock().await;

        let account = self.find_account(&guard, &card)?.clone();

        let transaction = Transaction {
            sender: guard.emission_account.clone(),
            recipient: account,
            amount,
            datetime: OffsetDateTime::now_utc(),
        };

        guard.transactions.push(transaction);

        self.notify(&guard);
        Ok(())
    }

    async fn list_transactions(
        &self,
    ) -> Result<Vec<Transaction>, BankOperationError> {
        let guard = self.lock().await;
        Ok(guard.transactions.clone())
    }

    async fn bank_emission(&self) -> Result<i64, BankOperationError> {
        let guard = self.lock().await;

        let emission_acc = &guard.emission_account;
        Ok(self.balance(&guard, emission_acc))
    }

    async fn new_card_token(
        &self,
        card: &CardNumber,
    ) -> Result<String, BankOperationError> {
        let mut guard = self.lock().await;

        let _ = self.find_account(&guard, card)?;
        let token = generate_token();

        guard.tokens.insert(token.clone(), card.clone());
        self.notify(&guard);
        Ok(token)
    }

    async fn get_account_by_token(
        &self,
        token: &str,
    ) -> Result<Account, BankOperationError> {
        let guard = self.lock().await;

        self.get_account_by_token(&guard, token)
    }
}
