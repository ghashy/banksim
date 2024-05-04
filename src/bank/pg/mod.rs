use anyhow::anyhow;
use anyhow::Context;
use argon2::password_hash::SaltString;
use argon2::PasswordHash;
use argon2::PasswordHasher;
use argon2::PasswordVerifier;
use axum::async_trait;
use banksim_api::init_payment::beneficiaries::Beneficiaries;
use deadpool::managed::Object;
use deadpool_postgres::Manager;
use deadpool_postgres::ManagerConfig;
use deadpool_postgres::Pool;
use futures::future::try_join_all;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use secrecy::ExposeSecret;
use secrecy::Secret;
use std::sync::Arc;
use tokio::sync::watch::{Receiver, Sender};
use tokio::task::JoinHandle;
use tokio_postgres::NoTls;
use tracing::Level;

use crate::config::DatabaseSettings;
use crate::cornucopia::queries::bank_queries;
use crate::domain::card_number::CardNumber;
use crate::middleware::Credentials;
use crate::Settings;

use super::backend::{BankDataBackend, InitBankDataBackend};
use super::generate_token;
use super::Account;
use super::BankOperationError;
use super::Transaction;

mod db_migration;

#[derive(Debug)]
pub struct PostgresStorage {
    pg_pool: Pool,
    notifier: Sender<()>,
    argon2_obj: argon2::Argon2<'static>,
    settings: Settings,
}

impl PostgresStorage {
    async fn balance(
        &self,
        db_client: &Object<Manager>,
        card: &CardNumber,
    ) -> Result<i64, BankOperationError> {
        let balance = bank_queries::get_account_balance()
            .bind(db_client, &card.as_ref())
            .one()
            .await
            .context("Failed to get balance from pg for an account")
            .map(|value| value.to_i64())?;
        balance.ok_or(BankOperationError::InternalError(anyhow::anyhow!(
            "Failed to parse Decimal to i64"
        )))
    }

    async fn account_transactions(
        &self,
        db_client: &Object<Manager>,
        card: &CardNumber,
    ) -> Result<Vec<Transaction>, BankOperationError> {
        bank_queries::list_account_transactions()
            .bind(db_client, &card.as_ref())
            .all()
            .await
            .context("Failed to get account transactions list from the pg")?
            .into_iter()
            .map(|t| {
                Ok(Transaction {
                    sender: Account {
                        username: t.sender_username,
                        card_number: t.sender_card_number.parse()?,
                        password: Secret::new(String::new()),
                        is_existing: t.sender_is_existing,
                    },
                    recipient: Account {
                        username: t.recipient_username,
                        card_number: t.recipient_card_number.parse()?,
                        password: Secret::new(String::new()),
                        is_existing: t.recipient_is_existing,
                    },
                    amount: t.amount,
                    datetime: t.created_at,
                })
            })
            .collect()
    }

    /// I want to notify my subscribers to update their accounts info
    /// after every bank lock
    fn notify(&self) {
        if let Err(e) = self.notifier.send(()) {
            tracing::warn!(
                "Failed to send bank state updated notification: {e}"
            );
        }
    }

    async fn find_account(
        &self,
        db_client: &Object<Manager>,
        card: &CardNumber,
    ) -> Result<Account, BankOperationError> {
        bank_queries::get_account()
            .bind(db_client, &card.as_ref())
            .opt()
            .await
            .context("Failed to find an account by card number in pg")?
            .ok_or(BankOperationError::AccountNotFound)
            .map(|acc| {
                Ok(Account {
                    username: acc.username,
                    card_number: acc.card_number.parse()?,
                    password: Secret::new(String::new()),
                    is_existing: acc.is_existing,
                })
            })?
    }

    async fn get_account_by_token(
        &self,
        db_client: &Object<Manager>,
        token: &str,
    ) -> Result<Account, BankOperationError> {
        bank_queries::get_account_by_token()
            .bind(db_client, &token)
            .opt()
            .await
            .context("Failed to find and account by token in pg")?
            .ok_or(BankOperationError::AccountNotFound)
            .map(|acc| {
                Ok(Account {
                    username: acc.username,
                    card_number: acc.card_number.parse()?,
                    password: Secret::new(String::new()),
                    is_existing: acc.is_existing,
                })
            })?
    }

    async fn emission_account(
        &self,
        db_client: &Object<Manager>,
    ) -> Result<Account, BankOperationError> {
        bank_queries::get_emission_account()
            .bind(db_client)
            .one()
            .await
            .context("Failed to fetch emission account from pg")
            .map(|acc| {
                Ok(Account {
                    username: acc.username,
                    card_number: acc.card_number.parse()?,
                    password: Secret::new(String::new()),
                    is_existing: acc.is_existing,
                })
            })?
    }
}

impl InitBankDataBackend for PostgresStorage {
    fn new(
        settings: &Settings,
        tx: Sender<()>,
    ) -> Arc<dyn BankDataBackend + Send + Sync> {
        let pg_pool = get_postgres_connection_pool(
            settings.database_settings.as_ref().unwrap(),
        );

        let argon2_obj = argon2::Argon2::new(
            argon2::Algorithm::Argon2id,
            argon2::Version::V0x13,
            // Params are good
            argon2::Params::new(15000, 2, 1, None).unwrap(),
        );

        let pg_pool_copy = pg_pool.clone();
        let settings_copy = settings.clone();
        let argon2_obj_copy = argon2_obj.clone();
        tokio::spawn(async move {
            db_migration::run_migration(&pg_pool_copy).await;
            let connection = pg_pool_copy.get().await.expect(
                "Failed to get postgres pool connection to run migrations",
            );
            let accounts_count = bank_queries::accounts_count()
                .bind(&connection)
                .one()
                .await
                .expect("Failed to check emission and store accounts");
            if accounts_count == 0 {
                tracing::info!("No system accounts found, creating ones...");
                let emission_account = Account {
                    card_number: CardNumber::generate(),
                    password: Secret::new(
                        hash_password_blocking(
                            argon2_obj_copy.clone(),
                            settings_copy.terminal_settings.password.clone(),
                        )
                        .await
                        .unwrap(),
                    ),
                    is_existing: true,
                    username: settings_copy.bank_username.clone(),
                };

                let store_account = Account {
                    card_number: CardNumber::generate(),
                    password: Secret::new(
                        hash_password_blocking(
                            argon2_obj_copy,
                            settings_copy.terminal_settings.password.clone(),
                        )
                        .await
                        .unwrap(),
                    ),
                    is_existing: true,
                    username: "store".to_string(),
                };
                bank_queries::insert_account()
                    .bind(
                        &connection,
                        &emission_account.username,
                        &emission_account.card_number.as_ref(),
                        &emission_account.password.expose_secret(),
                    )
                    .await
                    .unwrap();

                bank_queries::insert_account()
                    .bind(
                        &connection,
                        &store_account.username,
                        &store_account.card_number.as_ref(),
                        &store_account.password.expose_secret(),
                    )
                    .await
                    .unwrap();
                tracing::info!("Successfully created system accounts!");
            } else {
                tracing::info!("System accounts already exists in db!");
            }
        });

        Arc::new(PostgresStorage {
            pg_pool,
            notifier: tx,
            argon2_obj,
            settings: settings.clone(),
        })
    }
}

#[async_trait]
impl BankDataBackend for PostgresStorage {
    async fn subscribe(&self) -> Receiver<()> {
        self.notifier.subscribe()
    }

    async fn authorize_system(
        &self,
        credentials: Credentials,
    ) -> Result<(), BankOperationError> {
        let argon2_obj = self.argon2_obj.clone();

        let db_client = self
            .pg_pool
            .get()
            .await
            .context("Failed to get a pg client from pg pool")?;
        let emission_account = bank_queries::get_emission_account()
            .bind(&db_client)
            .one()
            .await
            .context("Failed to get emission account from pg")?;

        verify_password_hash_blocking(
            Secret::new(emission_account.password_hash),
            credentials.password,
            argon2_obj,
        )
        .await?;

        if emission_account.username.eq(&credentials.username) {
            Ok(())
        } else {
            Err(BankOperationError::NotAuthorized)
        }
    }

    async fn add_account(
        &self,
        username: &str,
        password: &Secret<String>,
    ) -> Result<CardNumber, BankOperationError> {
        let argon2_obj = self.argon2_obj.clone();
        let password = password.clone();
        let password_hash =
            hash_password_blocking(argon2_obj, password).await?;

        let db_client = self
            .pg_pool
            .get()
            .await
            .context("Failed to get a pg client from pg pool")?;
        let card_number = CardNumber::generate();
        bank_queries::insert_account()
            .bind(&db_client, &username, &card_number.as_ref(), &password_hash)
            .await
            .context("Failed to insert a new account to pg")?;

        self.notify();
        Ok(card_number)
    }

    async fn delete_account(
        &self,
        card: &CardNumber,
    ) -> Result<(), BankOperationError> {
        let db_client = self
            .pg_pool
            .get()
            .await
            .context("Failed to get a pg client from pg pool")?;
        if !bank_queries::is_account_exists()
            .bind(&db_client, &card.as_ref())
            .one()
            .await
            .context("Failed to fetch account info from pg")?
        {
            return Err(BankOperationError::AccountNotFound);
        }

        bank_queries::mark_account_as_deleted()
            .bind(&db_client, &card.as_ref())
            .await
            .context("Failed to delete account from pg")?;
        self.notify();
        Ok(())
    }

    async fn list_accounts(
        &self,
    ) -> Result<
        Vec<crate::domain::responses::system_api::Account>,
        BankOperationError,
    > {
        let db_client = self
            .pg_pool
            .get()
            .await
            .context("Failed to get a pg client from pg pool")?;
        let accounts = bank_queries::get_accounts()
            .bind(&db_client)
            .all()
            .await
            .context("Failed to insert a new account to pg")?
            .into_iter()
            .map(|acc| async move {
                let card_number = acc.card_number.parse()?;
                Ok::<
                    crate::domain::responses::system_api::Account,
                    BankOperationError,
                >(
                    crate::domain::responses::system_api::Account {
                        card_number,
                        balance: acc.balance.to_i64().ok_or(
                            BankOperationError::InternalError(anyhow!(
                                "Failed to parse Decimal into i64"
                            )),
                        )?,
                        transactions: Vec::new(),
                        exists: acc.is_existing,
                        tokens: acc.tokens.into_iter().flatten().collect(),
                        username: acc.username,
                    },
                )
            });
        let accounts = try_join_all(accounts).await?;
        let mut result = Vec::with_capacity(accounts.len());
        for mut account in accounts.into_iter() {
            // Skip store and emission accounts
            if account.username.eq(&self.settings.bank_username)
                || account.username.eq("store")
            {
                continue;
            }
            let transactions = self
                .account_transactions(&db_client, &account.card_number)
                .await?;
            account.transactions = transactions;
            result.push(account);
        }
        result.sort_by(|acc1, acc2| acc1.username.cmp(&acc2.username));
        Ok(result)
    }

    async fn authorize_account(
        &self,
        card: &CardNumber,
        password: &Secret<String>,
    ) -> Result<Account, BankOperationError> {
        let argon2_obj = self.argon2_obj.clone();

        let db_client = self
            .pg_pool
            .get()
            .await
            .context("Failed to get a pg client from pg pool")?;
        let account = bank_queries::get_account()
            .bind(&db_client, &card.as_ref())
            .one()
            .await
            .context("Failed to get account from pg")?;

        verify_password_hash_blocking(
            Secret::new(account.password_hash.clone()),
            password.clone(),
            argon2_obj,
        )
        .await?;

        let account = account.try_into()?;
        Ok(account)
    }

    async fn find_account(
        &self,
        card: &CardNumber,
    ) -> Result<Account, BankOperationError> {
        let db_client = self
            .pg_pool
            .get()
            .await
            .context("Failed to get a pg client from pg pool")?;
        self.find_account(&db_client, card).await
    }

    async fn get_store_account(&self) -> Result<Account, BankOperationError> {
        let db_client = self
            .pg_pool
            .get()
            .await
            .context("Failed to get a pg client from pg pool")?;
        bank_queries::get_store_account()
            .bind(&db_client)
            .one()
            .await
            .map(|acc| {
                Ok(Account {
                    username: acc.username,
                    card_number: acc.card_number.parse()?,
                    password: Secret::new(String::new()),
                    is_existing: acc.is_existing,
                })
            })
            .context("Failed to find an account by card number in pg")?
    }

    async fn store_balance(&self) -> Result<i64, BankOperationError> {
        let store_acc = self.get_store_account().await?;
        let db_client = self
            .pg_pool
            .get()
            .await
            .context("Failed to get a pg client from pg pool")?;
        self.balance(&db_client, &store_acc.card()).await
    }

    async fn balance(
        &self,
        card: &CardNumber,
    ) -> Result<i64, BankOperationError> {
        let db_client = self
            .pg_pool
            .get()
            .await
            .context("Failed to get a pg client from pg pool")?;
        self.balance(&db_client, card).await
    }

    #[tracing::instrument(
        name = "Try to create new simple transaction",
        skip(self)
    )]
    async fn new_transaction(
        &self,
        sender: &CardNumber,
        recipient: &CardNumber,
        amount: i64,
    ) -> Result<(), BankOperationError> {
        let db_client = self
            .pg_pool
            .get()
            .await
            .context("Failed to get a pg client from pg pool")?;
        match bank_queries::create_transaction()
            .bind(&db_client, &sender.as_ref(), &recipient.as_ref(), &amount)
            .await
        {
            Ok(_) => (),
            Err(e) => {
                tracing::error!("Failed to create transaction: {e}");
                if let Some(db_error) = e.as_db_error() {
                    if db_error.message().eq("Not enough funds") {
                        return Err(BankOperationError::NotEnoughFunds);
                    }
                    match db_error.message() {
                        "Not enough funds" => return Err(BankOperationError::NotEnoughFunds),
                        "Amount must be greater than 0" => return Err(BankOperationError::BadTransaction),
                        "Sender and recipient cannot be the same" => return Err(BankOperationError::BadTransaction),
                        "Sender or recipient account does not exist or is not active" => return Err(BankOperationError::AccountNotFound),
                        _ => ()
                    }
                }
                return Err(BankOperationError::UnexpectedError);
            }
        }

        self.notify();
        Ok(())
    }

    #[tracing::instrument(name = "Try create new split transaction", skip_all)]
    async fn new_split_transaction(
        &self,
        sender: &CardNumber,
        amount: i64,
        beneficiaries: &Beneficiaries,
    ) -> Result<(), BankOperationError> {
        beneficiaries
            .validate()
            .map_err(|_| BankOperationError::BadTransaction)?;

        let db_client = self
            .pg_pool
            .get()
            .await
            .context("Failed to get a pg client from pg pool")?;

        let mut bfc = Vec::with_capacity(beneficiaries.count());
        for (token, part) in beneficiaries.iter_tokens() {
            let acc = self.get_account_by_token(&db_client, token).await?;
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

        // Find sender
        let _ = self.find_account(&db_client, sender).await?;

        if self.balance(&db_client, &sender).await? < amount {
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
            self.new_transaction(sender, &recipient.card_number, amount)
                .await?;
        }

        self.notify();
        Ok(())
    }

    async fn open_credit(
        &self,
        card: &CardNumber,
        amount: i64,
    ) -> Result<(), BankOperationError> {
        let db_client = self
            .pg_pool
            .get()
            .await
            .context("Failed to get a pg client from pg pool")?;

        let emission_account = self.emission_account(&db_client).await?;
        let _ = self
            .new_transaction(&emission_account.card_number, card, amount)
            .await?;
        self.notify();
        Ok(())
    }

    async fn list_transactions(
        &self,
    ) -> Result<Vec<Transaction>, BankOperationError> {
        Ok(Vec::new())
    }

    async fn bank_emission(&self) -> Result<i64, BankOperationError> {
        let db_client = self
            .pg_pool
            .get()
            .await
            .context("Failed to get a pg client from pg pool")?;

        let emission_account = self.emission_account(&db_client).await?;
        self.balance(&db_client, &emission_account.card_number)
            .await
    }

    async fn new_card_token(
        &self,
        card: &CardNumber,
    ) -> Result<String, BankOperationError> {
        let db_client = self
            .pg_pool
            .get()
            .await
            .context("Failed to get a pg client from pg pool")?;

        let token = generate_token();

        // Check that card exists
        let _ = self.find_account(&db_client, card).await?;
        bank_queries::insert_token()
            .bind(&db_client, &card.as_ref(), &token)
            .await
            .context("Failed to insert new card token into pg")?;
        Ok(token)
    }

    async fn get_account_by_token(
        &self,
        token: &str,
    ) -> Result<Account, BankOperationError> {
        let db_client = self
            .pg_pool
            .get()
            .await
            .context("Failed to get a pg client from pg pool")?;
        self.get_account_by_token(&db_client, token).await
    }
}

pub async fn verify_password_hash_blocking(
    expected_password_hash: Secret<String>,
    password_candidate: Secret<String>,
    argon2_obj: argon2::Argon2<'static>,
) -> Result<(), BankOperationError> {
    spawn_blocking_with_tracing(move || {
        verify_password_hash(
            expected_password_hash,
            password_candidate,
            argon2_obj,
        )
    })
    .await
    .map_err(|_| BankOperationError::NotAuthorized)?
}

pub async fn hash_password_blocking(
    argon2_obj: argon2::Argon2<'static>,
    password: Secret<String>,
) -> Result<String, BankOperationError> {
    let hash = spawn_blocking_with_tracing(move || {
        hash_password(&password, argon2_obj)
    })
    .await
    .context("Failed to join thread")
    .map_err(BankOperationError::InternalError)??;
    Ok(hash)
}

/// We can now easily reach for it every time we need to offload
/// some CPU-intensive computation to a dedicated threadpool.
pub fn spawn_blocking_with_tracing<F, R>(f: F) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let current_span = tracing::Span::current();
    tokio::task::spawn_blocking(move || current_span.in_scope(f))
}

#[tracing::instrument(name = "Performing hashing of password", skip_all)]
fn hash_password(
    password: &Secret<String>,
    argon2: argon2::Argon2,
) -> Result<String, BankOperationError> {
    let salt = SaltString::generate(&mut rand::thread_rng());
    Ok(argon2
        .hash_password(password.expose_secret().as_bytes(), &salt)
        .context("Failed to hash password")
        .map_err(BankOperationError::InternalError)?
        .to_string())
}

#[tracing::instrument(level = Level::TRACE, name = "Verify password hash", skip_all)]
fn verify_password_hash(
    expected_password_hash: Secret<String>,
    password_candidate: Secret<String>,
    argon2: argon2::Argon2,
) -> Result<(), BankOperationError> {
    let expected_password_hash =
        PasswordHash::new(&expected_password_hash.expose_secret())
            .context("Failed to parse hash in PHC string format.")
            .map_err(BankOperationError::InternalError)?;
    argon2
        .verify_password(
            password_candidate.expose_secret().as_bytes(),
            &expected_password_hash,
        )
        .context("Invalid password")
        .map_err(BankOperationError::InternalError)
}

pub fn get_postgres_connection_pool(configuration: &DatabaseSettings) -> Pool {
    let pg_config = get_pg_conf(configuration);
    let connector = NoTls;
    let manager_config = ManagerConfig {
        recycling_method: deadpool_postgres::RecyclingMethod::Fast,
    };
    let manager = Manager::from_config(pg_config, connector, manager_config);
    let pool = Pool::builder(manager).max_size(16).build().unwrap();
    pool
}

fn get_pg_conf(configuration: &DatabaseSettings) -> tokio_postgres::Config {
    let mut config = tokio_postgres::Config::new();
    config.user(&configuration.username);
    config.dbname(&configuration.database_name);
    config.host(&configuration.host);
    config.password(&configuration.password.expose_secret());
    config
}
