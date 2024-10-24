use actix_cors::Cors;
use actix_extensible_rate_limit::backend::memory::InMemoryBackend;
use actix_extensible_rate_limit::backend::{
    SimpleInputFunctionBuilder, SimpleInputFuture, SimpleOutput,
};
use actix_extensible_rate_limit::{HeaderCompatibleOutput, RateLimiter};
use actix_web::dev::ServiceRequest;
use actix_web::HttpResponse;
use std::time::Duration;
use tracing::{error, info};

pub fn rate_limiter(
    rate_limit_backend: InMemoryBackend,
    max_requests: u64,
    limit_duration: u64,
) -> RateLimiter<InMemoryBackend, SimpleOutput, impl Fn(&ServiceRequest) -> SimpleInputFuture + Sized>
{
    let input = SimpleInputFunctionBuilder::new(Duration::from_secs(limit_duration), max_requests)
        .real_ip_key()
        .build();

    RateLimiter::builder(rate_limit_backend.clone(), input)
        .add_headers()
        .request_denied_response(|res| {
            HttpResponse::TooManyRequests()
                .insert_header(("x-ratelimit-reset", res.seconds_until_reset().to_string()))
                .insert_header(("x-ratelimit-limit", res.limit().to_string()))
                .insert_header(("x-ratelimit-remaining", res.remaining().to_string()))
                .body("Too many requests, please try again later")
        })
        .build()
}

pub fn rate_limiter_data(
    limit_env: (&str, &str),
    duration_env: (&str, &str),
) -> (InMemoryBackend, u64, u64) {
    info!("Setting up rate limiter");
    let rate_limit_backend = InMemoryBackend::builder().build();
    let max_requests = tosic_utils::prelude::env!(limit_env.0, limit_env.1)
        .parse()
        .unwrap_or_else(|e| {
            error!("Failed to parse {}: {}", duration_env.0, e);
            10
        });
    let limit_duration = tosic_utils::prelude::env!(duration_env.0, duration_env.1)
        .parse()
        .unwrap_or_else(|e| {
            error!("Failed to parse {}: {}", duration_env.0, e);
            60
        });

    (rate_limit_backend, max_requests, limit_duration)
}

pub fn cors() -> Cors {
    Cors::permissive().supports_credentials().allow_any_header()
}
