//! All endpoints that this API serves, the structure of this module is built as closely as possible to the actual endpoints path

use actix_web::web;
use utoipa::OpenApi;

pub(crate) mod api;
pub(crate) mod health;
mod not_found;

pub(crate) fn index_scope() -> impl actix_web::dev::HttpServiceFactory {
    web::scope("")
        .service(health::health)
        .default_service(web::to(not_found::not_found))
}