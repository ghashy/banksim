use std::ops::DerefMut;

use deadpool_postgres::Pool;
use refinery::embed_migrations;

embed_migrations!("./migrations");

pub async fn run_migration(pool: &Pool) {
    let mut connection = pool
        .get()
        .await
        .expect("Failed to get postgres pool connection to run migrations");
    let client = connection.deref_mut().deref_mut();

    let report = match migrations::runner().run_async(client).await {
        Ok(r) => r,
        Err(e) => {
            tracing::warn!("Can't run migration on db: {}", e);
            return;
        }
    };

    if report.applied_migrations().is_empty() {
        tracing::info!("No migrations applied");
    }

    for migration in report.applied_migrations() {
        tracing::info!("Migration: {}", migration);
    }
}
