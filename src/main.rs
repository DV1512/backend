use crate::config::{cors, rate_limiter, rate_limiter_data};
use crate::init_env::init_env;
use crate::logging::init_tracing;
use crate::server_error::ServerError;
use crate::state::{app_state, AppState};
use actix_web::{web, HttpServer};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use surrealdb::engine::local::Db;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use tracing::info;
use tracing_actix_web::TracingLogger;

mod auth;
mod config;
mod init_env;
mod logging;
mod server_error;
mod state;

static INTERNAL_DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Record {
    id: Thing,
}

#[derive(Debug, Serialize, Deserialize, PartialOrd, Eq, PartialEq, Clone)]
pub(crate) struct PaginationResponse {
    limit: Option<u64>,
    offset: Option<u64>,
    total: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub(crate) struct PaginationRequest {
    limit: Option<u64>,
    offset: Option<u64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct CountResponse {
    pub count: u64,
}

#[tracing::instrument]
async fn init_internal_db() -> Result<(), ServerError> {
    let db_url = tosic_utils::prelude::env!("SURREALDB_URL");
    let db_user = tosic_utils::prelude::env!("SURREALDB_USER");
    let db_pass = tosic_utils::prelude::env!("SURREALDB_PASS");

    INTERNAL_DB.connect::<Ws>(db_url).await?;

    INTERNAL_DB
        .signin(surrealdb::opt::auth::Root {
            username: &db_user,
            password: &db_pass,
        })
        .await?;

    let namespace = tosic_utils::prelude::env!("SURREALDB_NAMESPACE", "default");
    let database = tosic_utils::prelude::env!("SURREALDB_DATABASE", "default");

    INTERNAL_DB.use_ns(namespace).use_db(database).await?;

    Ok(())
}

#[actix::main]
async fn main() -> Result<(), ServerError> {
    init_env()?;

    init_tracing()?;
    info!("Logging initialized");

    init_internal_db().await?;

    let port = tosic_utils::prelude::env!("PORT", "9999");
    let frontend_url = tosic_utils::prelude::env!("FRONTEND_URL", "http://localhost:42069");
    let (rate_limit_backend, max_requests, limit_duration) =
        rate_limiter_data(("LIMIT", "10"), ("LIMIT_DURATION", "60"));

    let state: web::Data<AppState<Db>> = app_state().await?;

    info!("Setting up server on port {}", port);
    HttpServer::new(move || {
        let cors = cors();
        let limiter = rate_limiter(rate_limit_backend.clone(), max_requests, limit_duration);

        actix_web::App::new()
            .wrap(cors)
            .wrap(limiter)
            .wrap(TracingLogger::default())
            .app_data(state.clone())
            .external_resource("frontend", frontend_url.clone())
    })
    .bind(format!("0.0.0.0:{port}"))?
    .run()
    .await?;

    Ok(())
}
