#![cfg(test)]

macro_rules! init_test_app {
    () => {{
        crate::server::setup().await?;

        let log_sender = crate::server::background_logger();

        let crate::server::rate_limiter::RateLimiterData {
            backend,
            max_requests,
            limit_duration,
        } = crate::server::rate_limiter::RateLimiterData::default();

        let state = crate::state::app_state().await?;

        let cors = crate::config::cors();
        let limiter = crate::config::rate_limiter(backend.clone(), max_requests, limit_duration);
        let logger = crate::middlewares::logger::LoggingMiddleware::new(log_sender.clone());
        let identity = actix_identity::IdentityMiddleware::builder()
            .login_deadline(Some(std::time::Duration::from_hours(1)))
            .build();

        let frontend_url = tosic_utils::prelude::env!("FRONTEND_URL", "http://localhost:5173");
        let base_url = tosic_utils::prelude::env!("BASE_URL", "http://localhost:9999");

        let key = actix_web::cookie::Key::generate();

        actix_web::test::init_service(crate::server::app!(
            state,
            limiter,
            logger,
            frontend_url,
            base_url,
            key,
            cors,
            identity
        ))
        .await
    }};
}

/// Constructs a actix test function, the test must return a `Result<(), ServerError>`.
macro_rules! actix_test {
    (fn $ident:ident() $block:block) => {
        #[actix::test]
        async fn $ident() -> Result<(), crate::server_error::ServerError> $block
    };
}

pub(crate) use {actix_test, init_test_app};
