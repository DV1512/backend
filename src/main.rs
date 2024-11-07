#![feature(async_closure)]
#![feature(duration_constructors)]

use crate::server::{server, setup};
use crate::server_error::ServerError;
use crate::state::AppState;
use helper_macros::generate_endpoint;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::sql::Thing;
use surrealdb::Surreal;

mod auth;
mod config;
mod dto;
mod endpoints;
mod error;
mod extractors;
mod init_env;
mod logging;
mod middlewares;
mod models;
mod server;
mod server_error;
mod services;
mod state;
mod swagger;
mod utils;

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
            username: db_user.as_str(),
            password: db_pass.as_str(),
        })
        .await?;

    let namespace = tosic_utils::prelude::env!("SURREALDB_NAMESPACE", "default");
    let database = tosic_utils::prelude::env!("SURREALDB_DATABASE", "default");

    INTERNAL_DB
        .use_ns(namespace.as_str())
        .use_db(database.as_str())
        .await?;

    Ok(())
}

#[actix::main]
async fn main() -> Result<(), ServerError> {
    setup().await?;

    let port = tosic_utils::prelude::env!("PORT", "9999");

    server!()
        .bind(format!("0.0.0.0:{port}"))?
        .bind(format!("[::1]:{port}"))?
        .run()
        .await?;

    Ok(())
}
