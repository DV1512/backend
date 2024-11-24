use actix_web::{web, Scope};
use utoipa::OpenApi;

pub(crate) mod callback;
pub(crate) mod login;

pub(crate) use {callback::*, login::*};

pub fn google_oauth_service() -> Scope {
    web::scope("/google")
        .service(google_login)
        .service(google_callback)
}

#[derive(OpenApi)]
#[openapi(paths(google_login, google_callback))]
pub(crate) struct GoogleApi;
