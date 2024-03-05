mod bank;
mod config;
mod html_gen;
mod interaction_sessions;
mod middleware;
mod routes;
mod startup;

pub mod domain;
pub mod tasks;
pub mod ws_tracing_subscriber;
pub mod cornucopia;

use std::error::Error;

pub use config::Settings;
pub use startup::Application;
use uuid::Uuid;

pub fn error_chain_fmt(
    e: &impl Error,
    f: &mut std::fmt::Formatter<'_>,
) -> std::fmt::Result {
    writeln!(f, "{}\n", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{}", cause)?;
        current = cause.source();
    }
    Ok(())
}

pub trait RemovableById {
    fn remove(&self, id: Uuid) -> Result<(), Box<dyn Error>>;
}
