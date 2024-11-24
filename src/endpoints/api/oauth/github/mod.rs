use actix_web::{web, Scope};
use utoipa::OpenApi;

pub(crate) mod callback;
pub(crate) mod login;

pub(crate) use self::{callback::*, login::*};

pub fn github_oauth_service() -> Scope {
    web::scope("/github").service(login).service(callback)
}

#[derive(OpenApi)]
#[openapi(paths(login, callback))]
pub(crate) struct GithubApi;
