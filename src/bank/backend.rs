use std::sync::Arc;

use axum::async_trait;
use banksim_api::init_payment::beneficiaries::Beneficiaries;
use secrecy::Secret;
use tokio::sync::watch::Receiver;
use tokio::sync::watch::Sender;

use crate::domain::card_number::CardNumber;
use crate::middleware::Credentials;
use crate::Settings;

use super::{Account, BankOperationError, Transaction};

pub trait InitBankDataBackend {
    fn new(
        settings: &Settings,
        tx: Sender<()>,
    ) -> Arc<dyn BankDataBackend + Send + Sync>;
}

#[async_trait]
pub trait BankDataBackend
where
    Self: std::fmt::Debug,
{
    async fn subscribe(&self) -> Receiver<()>;
    async fn authorize_system(
        &self,
        credentials: Credentials,
    ) -> Result<(), BankOperationError>;
    async fn add_account(
        &self,
        username: &str,
        password: &Secret<String>,
    ) -> Result<CardNumber, BankOperationError>;
    async fn delete_account(
        &self,
        card: &CardNumber,
    ) -> Result<(), BankOperationError>;
    async fn list_accounts(
        &self,
    ) -> Result<
        Vec<crate::domain::responses::system_api::Account>,
        BankOperationError,
    >;
    async fn authorize_account(
        &self,
        card: &CardNumber,
        password: &Secret<String>,
    ) -> Result<Account, BankOperationError>;
    async fn find_account(
        &self,
        card: &CardNumber,
    ) -> Result<Account, BankOperationError>;
    async fn get_store_account(&self) -> Result<Account, BankOperationError>;
    async fn store_balance(&self) -> Result<i64, BankOperationError>;
    async fn balance(
        &self,
        card: &CardNumber,
    ) -> Result<i64, BankOperationError>;
    async fn new_transaction(
        &self,
        sender: &CardNumber,
        recipient: &CardNumber,
        amount: i64,
    ) -> Result<(), BankOperationError>;
    async fn new_split_transaction(
        &self,
        sender: &CardNumber,
        amount: i64,
        beneficiaries: &Beneficiaries,
    ) -> Result<(), BankOperationError>;
    async fn open_credit(
        &self,
        card: &CardNumber,
        amount: i64,
    ) -> Result<(), BankOperationError>;
    async fn list_transactions(
        &self,
    ) -> Result<Vec<Transaction>, BankOperationError>;
    async fn bank_emission(&self) -> Result<i64, BankOperationError>;
    async fn new_card_token(
        &self,
        card: &CardNumber,
    ) -> Result<String, BankOperationError>;
    async fn get_account_by_token(
        &self,
        token: &str,
    ) -> Result<Account, BankOperationError>;
}
