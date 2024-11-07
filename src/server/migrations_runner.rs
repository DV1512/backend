use crate::server::db::INTERNAL_DB;
use surrealdb_migrations::MigrationRunner;
use tracing::{error, info};

pub async fn migrations_runner() {
    match MigrationRunner::new(&INTERNAL_DB).up().await {
        Ok(_) => info!("Migrations ran successfully"),
        Err(e) => error!("Error running migrations: {}", e),
    }
}
