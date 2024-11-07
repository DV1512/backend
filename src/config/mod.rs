use actix_cors::Cors;
use actix_extensible_rate_limit::backend::memory::InMemoryBackend;
use actix_extensible_rate_limit::backend::{
    SimpleInputFunctionBuilder, SimpleInputFuture, SimpleOutput,
};
use actix_extensible_rate_limit::{HeaderCompatibleOutput, RateLimiter};
use actix_web::dev::ServiceRequest;
use actix_web::HttpResponse;
use std::time::Duration;

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

pub fn cors() -> Cors {
    Cors::permissive().supports_credentials().allow_any_header()
}
