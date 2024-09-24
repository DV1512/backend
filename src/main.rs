use crate::auth::oauth::oauth_service;
use crate::auth::users::user_service;
use crate::config::{cors, rate_limiter, rate_limiter_data};
use crate::init_env::init_env;
use crate::logging::init_tracing;
use crate::middlewares::auth::AuthMiddleware;
use crate::middlewares::logger::{LogEntry, LoggingMiddleware};
use crate::server_error::ServerError;
use crate::state::{app_state, AppState};
use actix_web::http::StatusCode;
use actix_web::{get, guard, http, HttpResponse, HttpResponseBuilder, HttpServer, Responder};
use apistos::{api_operation, web, SwaggerUIConfig};
use apistos::app::OpenApiWrapper;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use surrealdb_migrations::MigrationRunner;
use tokio::sync::mpsc;
use tracing::{error, info};
use tracing_actix_web::TracingLogger;
use apistos_plugins::ui::{UIPlugin, UIPluginConfig};
use oauth2::HttpRequest;

mod auth;
mod config;
mod init_env;
mod logging;
mod middlewares;
mod server_error;
mod state;
mod spec;

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

//#[get("/health")]
#[api_operation(
    tag = "health",
    summary = "Health check",
    description = r###"Checks the health of the API"###,
    operation_id = "health_check",
)]
async fn health_check() -> impl Responder { HttpResponse::Ok() }

#[api_operation(skip)]
pub(crate) async fn swagger() -> impl Responder {
    let swagger = Box::new(SwaggerUIConfig::new(&"/swagger")).build("/openapi.json");
    HttpResponseBuilder::new(StatusCode::OK)
        .content_type("text/html; charset=utf-8")
        .body(swagger.to_html())
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
        rate_limiter_data(("LIMIT", "10"), ("LIMIT_DURATION", "60"));

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
        let spec = spec::api_spec();

        actix_web::App::new()
            .document(spec)
            .app_data(state.clone())
            .wrap(TracingLogger::default()) // this is logging using tracing
            .wrap(logger) // this is database logging
            //.wrap(AuthMiddleware) // proof of concept, this should be moved into each individual service we want to secure with auth
            .external_resource("frontend", frontend_url.clone())
            .external_resource("base_url", base_url.clone())
            .service(user_service())
            .service(oauth_service())
            .service(web::resource("/health").route(web::get().to(health_check)))
            .service(web::resource("/swagger").route(web::get().to(swagger)))
            .wrap(cors)
            .wrap(limiter)
            .build("/openapi.json")
    })
    .bind(format!("0.0.0.0:{port}"))?
    .bind(format!("[::1]:{port}"))?
    .run()
    .await?;

    Ok(())
}
