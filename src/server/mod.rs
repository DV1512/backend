
pub mod background_logger;
pub mod db;
pub mod migrations_runner;
pub mod rate_limiter;
pub(crate) mod test;

#[cfg(not(test))]
use tracing::info;
pub use migrations_runner::*;

pub use background_logger::*;
use crate::init_env::init_env;
use crate::server_error::ServerError;
use db::init_internal_db;
#[cfg(not(test))]
use crate::logging::init_tracing;


pub async fn setup() -> Result<(), ServerError> {
    init_env()?;

    #[cfg(not(test))]
    {
        init_tracing()?;
        info!("Logging initialized");
    }

    init_internal_db().await?;

    migrations_runner().await;

    Ok(())
}

macro_rules! app {
    ($state:ident, $limiter:ident, $logger:ident, $frontend_url:ident, $base_url:ident, $key:ident, $cors:ident, $identity:ident) => {
        actix_web::App::new()
            .app_data($state.clone())
            //.wrap(AuthMiddleware) // proof of concept, this should be moved into each individual service we want to secure with auth
            .external_resource("frontend", $frontend_url.clone())
            .external_resource("base_url", $base_url.clone())
            .external_resource("llm", tosic_utils::prelude::env!("LLM_BACKEND", "http://localhost:8000"))
            .service(crate::endpoints::index_scope($limiter, $logger))
            .wrap($cors)
            .wrap($identity)
            .wrap(
                actix_session::SessionMiddleware::builder(actix_session::storage::CookieSessionStore::default(), $key.clone())
                    .cookie_same_site(actix_web::cookie::SameSite::None)
                    .session_lifecycle(actix_session::config::PersistentSession::default().session_ttl(actix_web::cookie::time::Duration::hours(1)))
                    .build(),
            )
    };
}
pub(crate) use app;

/// A macro for creating the actix web server. this can be used in a `main` function or in tests to setup the basic configuration of the server, it does not start the server!
macro_rules! server {
    () => {{
        let log_sender = crate::server::background_logger();

        let crate::server::rate_limiter::RateLimiterData {
            backend,
            max_requests,
            limit_duration,
        } = crate::server::rate_limiter::RateLimiterData::default();

        let state = crate::state::app_state().await?;

        actix_web::HttpServer::new(move || {
            let cors = crate::config::cors();
            let limiter =
                crate::config::rate_limiter(backend.clone(), max_requests, limit_duration);
            let logger = crate::middlewares::logger::LoggingMiddleware::new(log_sender.clone());
            let identity = actix_identity::IdentityMiddleware::builder()
                .login_deadline(Some(std::time::Duration::from_hours(1)))
                .build();

            let frontend_url = tosic_utils::prelude::env!("FRONTEND_URL", "http://localhost:5173");
            let base_url = tosic_utils::prelude::env!("BASE_URL", "http://localhost:9999");

            let key = actix_web::cookie::Key::generate();

            crate::server::app!(
                state,
                limiter,
                logger,
                frontend_url,
                base_url,
                key,
                cors,
                identity
            )
        })
    }};
}
pub(crate) use server;

