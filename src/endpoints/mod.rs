#![allow(unused_imports)]
//! All endpoints that this API serves, the structure of this module is built as closely as possible to the actual endpoints path

use actix_extensible_rate_limit::backend::memory::InMemoryBackend;
use actix_extensible_rate_limit::backend::{SimpleInputFuture, SimpleOutput};
use actix_extensible_rate_limit::RateLimiter;
use actix_web::dev::ServiceRequest;
use actix_web::web;
use utoipa::OpenApi;

pub(crate) mod api;
pub(crate) mod health;
mod not_found;

use crate::middlewares::logger::LoggingMiddleware;
pub(crate) use api::*;
pub(crate) use health::*;

pub(crate) fn index_scope(
    limiter: RateLimiter<
        InMemoryBackend,
        SimpleOutput,
        impl Fn(&ServiceRequest) -> SimpleInputFuture + Sized + 'static,
    >,
    logger: LoggingMiddleware,
) -> impl actix_web::dev::HttpServiceFactory {
    web::scope("")
        .service(health::health)
        .service(api(limiter, logger))
        .default_service(web::to(not_found::not_found))
}
