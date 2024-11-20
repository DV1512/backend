use oauth::oauth_service;
use user::user_service;
use crate::middlewares::logger::LoggingMiddleware;
use crate::swagger::{ApiDocs, DocsV1};
use actix_extensible_rate_limit::backend::memory::InMemoryBackend;
use actix_extensible_rate_limit::backend::{SimpleInputFuture, SimpleOutput};
use actix_extensible_rate_limit::RateLimiter;
use actix_web::dev::ServiceRequest;
use actix_web::middleware::NormalizePath;
use actix_web::web;
use tracing_actix_web::TracingLogger;
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;
use utoipa_redoc::{Redoc, Servable};
use utoipa_scalar::{Scalar, Servable as OtherServable};
use utoipa_swagger_ui::{Config, SwaggerUi};

pub(crate) mod oauth;
pub(crate) mod user;

pub(crate) use oauth::*;
pub(crate) use user::*;

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
pub(crate) fn api(
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
