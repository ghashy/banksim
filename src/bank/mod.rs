use std::ops::Deref;
use std::sync::Arc;

use rand::{distributions::Alphanumeric, Rng};
use secrecy::Secret;
use serde::Serialize;
use time::format_description::well_known::iso8601;
use time::format_description::well_known::iso8601::TimePrecision;
use time::format_description::well_known::Iso8601;
use time::OffsetDateTime;
use tokio::sync::TryLockError;

use crate::cornucopia::queries::bank_queries::GetAccount;
use crate::domain::card_number::CardNumber;
use crate::error_chain_fmt;
use crate::Settings;

use self::backend::BankDataBackend;

mod backend;
pub mod memory;
pub mod pg;

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
    #[error("Internal error")]
    InternalError(#[from] anyhow::Error),
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
    inner: Arc<dyn BankDataBackend + Send + Sync>,
}

impl Deref for Bank {
    type Target = Arc<dyn BankDataBackend + Send + Sync>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Bank {
    pub fn new<T: backend::InitBankDataBackend>(settings: &Settings) -> Self {
        let (tx, _) = tokio::sync::watch::channel(());
        let bank = T::new(settings, tx);
        Bank { inner: bank }
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
    username: String,
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

impl TryFrom<GetAccount> for Account {
    type Error = anyhow::Error;
    fn try_from(value: GetAccount) -> Result<Self, Self::Error> {
        Ok(Account {
            username: value.username,
            card_number: value.card_number.parse()?,
            password: Secret::new(String::new()),
            is_existing: value.is_existing,
        })
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
    use crate::config::TerminalSettings;

    use self::memory::MemoryStorage;

    use super::*;
    use banksim_api::init_payment::beneficiaries::Beneficiaries;
    use rs_merkle::{Hasher, MerkleTree};
    use rust_decimal::{prelude::FromPrimitive, Decimal};
    use url::Url;
    use uuid::Uuid;

    fn make_bank() -> Bank {
        let url: Url = "http://google.com".parse().unwrap();
        let settings = Settings {
            data_backend_type: crate::config::DataBackendType::Mem,
            database_settings: None,
            port: 15100,
            addr: "localhost".to_string(),
            terminal_settings: TerminalSettings {
                terminal_key: Uuid::new_v4(),
                success_url: url.clone(),
                fail_url: url.clone(),
                success_add_card_url: url.clone(),
                fail_add_card_url: url.clone(),
                notification_url: url.clone(),
                password: Secret::new(String::from("pass")),
                send_notification_finish_authorize: false,
                send_notification_completed: false,
                send_notification_reversed: false,
            },
            bank_username: "test_bank".to_string(),
        };
        Bank::new::<MemoryStorage>(&settings)
    }

    #[tokio::test]
    async fn split_transaction_success() {
        let bank = make_bank();
        let payer_card = bank
            .add_account("payer", &Secret::new("pass".to_string()))
            .await
            .unwrap();
        bank.open_credit(&payer_card, 500).await.unwrap();

        let store = bank.get_store_account().await.unwrap().card();
        let bfc1 = bank
            .add_account("bfc1", &Secret::new("pass".to_string()))
            .await
            .unwrap();
        let bfc2 = bank
            .add_account("bfc2", &Secret::new("pass".to_string()))
            .await
            .unwrap();

        let store_tok = bank.new_card_token(&store).await.unwrap();
        let bfc1_tok = bank.new_card_token(&bfc1.clone()).await.unwrap();
        let bfc2_tok = bank.new_card_token(&bfc2.clone()).await.unwrap();

        let bfc =
            Beneficiaries::builder(store_tok, Decimal::from_f32(0.37).unwrap())
                .add(bfc1_tok, Decimal::from_f32(0.31).unwrap())
                .add(bfc2_tok, Decimal::from_f32(0.32).unwrap())
                .build()
                .unwrap();

        bank.new_split_transaction(&payer_card, 256, &bfc)
            .await
            .unwrap();

        assert_eq!(
            bank.balance(&bank.get_store_account().await.unwrap().card())
                .await
                .unwrap(),
            95
        );
        assert_eq!(bank.balance(&bfc1).await.unwrap(), 79);
        assert_eq!(bank.balance(&bfc2).await.unwrap(), 82);
        assert_eq!(bank.balance(&payer_card).await.unwrap(), 244);
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
