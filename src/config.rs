use std::path::Path;

use anyhow::Context;
use config::FileFormat;
use secrecy::Secret;
use serde::Deserialize;
use url::Url;

#[derive(Deserialize, Debug, Clone)]
pub enum DataBackendType {
    Pg,
    Mem,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Settings {
    #[serde(default = "data_backend_type")]
    pub data_backend_type: DataBackendType,
    pub data_settings: Option<DatabaseSettings>,
    pub port: u16,
    pub addr: String,
    pub terminal_settings: TerminalSettings,
    pub bank_username: String,
}

impl Settings {
    pub fn load_configuration() -> Result<Settings, anyhow::Error> {
        let config_file = std::env::var("BANKSIM_CONFIG_FILE")
            .expect("BANKSIM_CONFIG_FILE var is unset!");

        config::Config::builder()
            .add_source(config::File::new(&config_file, FileFormat::Yaml))
            .build()?
            .try_deserialize()
            .context("Failed to build config from local config file.")
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct TerminalSettings {
    // Currently unused
    pub terminal_key: uuid::Uuid,

    pub success_url: Url,
    pub fail_url: Url,
    pub success_add_card_url: Url,
    pub fail_add_card_url: Url,
    pub notification_url: Url,
    #[serde(default = "terminal_password")]
    pub password: Secret<String>,
    /// Определяет, будет ли отправлена нотификация на выполнение метода FinishAuthorize
    pub send_notification_finish_authorize: bool,
    /// Определяет, будет ли отправлена нотификация на выполнение метода AttachCard
    pub send_notification_completed: bool,
    /// Определяет, будет ли отправлена нотификация на выполнение метода Cancel
    pub send_notification_reversed: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DatabaseSettings {
    pub username: String,
    pub database_name: String,
    pub host: String,
    #[serde(default = "pg_password")]
    pub password: Secret<String>,
}

fn terminal_password() -> Secret<String> {
    Secret::new(
        load_value_from_file(
            std::env::var("TERMINAL_PASSWORD_FILE")
                .expect("TERMINAL_PASSWORD_FILE var is unset!"),
        )
        .expect("Can't read terminal password file!"),
    )
}

fn pg_password() -> Secret<String> {
    Secret::new(
        load_value_from_file(
            std::env::var("POSTGRES_PASSWORD_FILE")
                .expect("POSTGRES_PASSWORD_FILE var is unset!"),
        )
        .expect("Can't read postgres password file!"),
    )
}

fn data_backend_type() -> DataBackendType {
    let value = std::env::var("DATA_BACKEND_TYPE")
        .expect("DATA_BACKEND_TYPE var is unset!");
    match value.as_str() {
        "postgres" => DataBackendType::Pg,
        "memory" => DataBackendType::Mem,
        _ => panic!(),
    }
}

fn load_value_from_file<T: AsRef<Path>>(
    path: T,
) -> Result<String, std::io::Error> {
    Ok(std::fs::read_to_string(path)?.trim().to_string())
}
