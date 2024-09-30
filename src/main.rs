#![feature(async_closure)]

use crate::auth::oauth::oauth_service;
use crate::auth::users::user_service;
use crate::config::{cors, rate_limiter, rate_limiter_data};
use crate::init_env::init_env;
use crate::logging::init_tracing;
use crate::middlewares::logger::{LogEntry, LoggingMiddleware};
use crate::server_error::ServerError;
use crate::state::{app_state, AppState};
use crate::swagger::{ApiDocs, DocsV1};
use actix_extensible_rate_limit::backend::memory::InMemoryBackend;
use actix_extensible_rate_limit::backend::{SimpleInputFuture, SimpleOutput};
use actix_extensible_rate_limit::RateLimiter;
use actix_web::dev::ServiceRequest;
use actix_web::middleware::NormalizePath;
use actix_web::{get, web, HttpResponse, HttpServer, Responder};
use api_forge::{ApiRequest, Request};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use surrealdb_migrations::MigrationRunner;
use tokio::sync::mpsc;
use tracing::{error, info};
use tracing_actix_web::TracingLogger;
use utoipa::OpenApi as OpenApiTrait;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as OtherServable};
use utoipa_swagger_ui::{Config, SwaggerUi};

mod auth;
mod config;
mod error;
mod init_env;
mod logging;
mod middlewares;
mod models;
mod server_error;
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

#[utoipa::path(
    responses(
        (status = 200, description = "Health check endpoint", body = String),
        (status = 424, description = "Database not responding", body = String),
    ),
    tag = "health",
)]
#[get("/health")]
pub async fn health_check() -> impl Responder {
    #[derive(Request, Serialize, Debug)]
    #[request(endpoint = "/health")]
    struct DbHealthCheck;

    let request = DbHealthCheck;

    let mut url = tosic_utils::prelude::env!("SURREALDB_URL");

    if !url.starts_with("http://") || !url.starts_with("https://") {
        url = format!("http://{}", url);
    }

    if let Err(err) = request.send_request(url.as_str(), None, None).await {
        error!("Database not responding, error: {}", err);

        HttpResponse::FailedDependency().body("Database not responding")
    } else {
        HttpResponse::Ok().body("Ok")
    }
}

/// All v1 API endpoints
fn v1_endpoints(
    limiter: RateLimiter<
        InMemoryBackend,
        SimpleOutput,
        impl Fn(&ServiceRequest) -> SimpleInputFuture + Sized + 'static,
    >,
    logger: LoggingMiddleware,
) -> impl actix_web::dev::HttpServiceFactory {
    web::scope("/v1")
        .wrap(TracingLogger::default()) // this is logging using tracing
        .wrap(logger) // this is database logging
        .service(user_service())
        .service(oauth_service())
        .wrap(limiter)
        .wrap(NormalizePath::default())
}

/// All API endpoints
fn api(
    limiter: RateLimiter<
        InMemoryBackend,
        SimpleOutput,
        impl Fn(&ServiceRequest) -> SimpleInputFuture + Sized + 'static,
    >,
    logger: LoggingMiddleware,
) -> impl actix_web::dev::HttpServiceFactory {
    web::scope("/api")
        .service(v1_endpoints(limiter, logger))
        .service(docs())
}

/// Documentation for only the v1 API. This does not include the docs for non `/api/v1` endpoints as that is done in `docs`
fn v1_docs() -> impl actix_web::dev::HttpServiceFactory {
    let openapi = DocsV1::openapi();
    let config = Config::from("/api/docs/v1/openapi.json");

    web::scope("/v1")
        .service(Redoc::with_url("/redoc", openapi.clone()))
        .service(
            SwaggerUi::new("/swagger/{_:.*}")
                .url("/openapi.json", openapi.clone())
                .config(config),
        )
        .service(RapiDoc::new("/api/docs/v1/openapi.json").path("/rapidoc"))
        .service(Scalar::with_url("/scalar", openapi.clone()))
}

/// Only real reason we have this is to be able to put scoped middlewares for the docs, for example we can add auth middleware to secure the docs
fn docs() -> impl actix_web::dev::HttpServiceFactory {
    let openapi = ApiDocs::openapi();
    let config = Config::from("/api/docs/openapi.json");

    web::scope("/docs")
        .service(Redoc::with_url("/redoc", openapi.clone()))
        .service(
            SwaggerUi::new("/swagger/{_:.*}")
                .url("/openapi.json", openapi.clone())
                .config(config),
        )
        .service(RapiDoc::new("/api/docs/openapi.json").path("/rapidoc"))
        .service(Scalar::with_url("/scalar", openapi.clone()))
        .service(v1_docs())
}

#[actix::main]
async fn main() -> Result<(), ServerError> {
    init_env()?;

    init_tracing()?;
    info!("Logging initialized");

    init_internal_db().await?;

    match MigrationRunner::new(&INTERNAL_DB).up().await {
        Ok(_) => info!("Migrations ran successfully"),
        Err(e) => error!("Error running migrations: {}", e),
    }

    let port = tosic_utils::prelude::env!("PORT", "9999");
    let frontend_url = tosic_utils::prelude::env!("FRONTEND_URL", "http://localhost:42069");
    let base_url = tosic_utils::prelude::env!("BASE_URL", "http://localhost:9999");

    let (rate_limit_backend, max_requests, limit_duration) =
        rate_limiter_data(("LIMIT", "100"), ("LIMIT_DURATION", "30"));

    let state = app_state().await?;

    let (log_sender, mut log_receiver) = mpsc::channel::<LogEntry>(100);

    tokio::spawn(async move {
        while let Some(log) = log_receiver.recv().await {
            // TODO: database logging
            // this is to not make a network call to the database for every request immediately and instead make it happen in the background
            info!("{:?}", log)
        }
    });

    info!("Setting up server on port {}", port);
    HttpServer::new(move || {
        let cors = cors();
        let limiter = rate_limiter(rate_limit_backend.clone(), max_requests, limit_duration);
        let logger = LoggingMiddleware::new(log_sender.clone());

        actix_web::App::new()
            .app_data(state.clone())
            //.wrap(AuthMiddleware) // proof of concept, this should be moved into each individual service we want to secure with auth
            .external_resource("frontend", frontend_url.clone())
            .external_resource("base_url", base_url.clone())
            .service(health_check)
            .service(api(limiter, logger))
            .wrap(cors)
    })
    .bind(format!("0.0.0.0:{port}"))?
    .bind(format!("[::1]:{port}"))?
    .run()
    .await?;

    Ok(())
}
