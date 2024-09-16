use crate::config::{cors, rate_limiter, rate_limiter_data};
use crate::init_env::init_env;
use crate::logging::init_tracing;
use crate::server_error::ServerError;
use actix_web::HttpServer;
use tracing::info;
use tracing_actix_web::TracingLogger;

mod config;
mod init_env;
mod logging;
mod server_error;

#[actix::main]
async fn main() -> Result<(), ServerError> {
    init_env()?;

    init_tracing()?;
    info!("Logging initialized");

    let port = tosic_utils::prelude::env!("PORT", "9999");
    let (rate_limit_backend, max_requests, limit_duration) =
        rate_limiter_data(("AUTH_LIMIT", "10"), ("AUTH_LIMIT_DURATION", "60"));

    info!("Setting up server on port {}", port);
    HttpServer::new(move || {
        let cors = cors();
        let limiter = rate_limiter(rate_limit_backend.clone(), max_requests, limit_duration);

        actix_web::App::new()
            .wrap(cors)
            .wrap(limiter)
            .wrap(TracingLogger::default())
    })
    .bind(format!("0.0.0.0:{port}"))?
    .run()
    .await?;

    Ok(())
}
