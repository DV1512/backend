use actix_web::guard::Acceptable;
use actix_web::{web, Scope};
use utoipa::OpenApi;

pub(crate) mod github;
pub(crate) mod google;
pub(crate) mod register;
pub(crate) mod revoke;
pub(crate) mod token;

pub(crate) use {github::*, google::*, register::*, revoke::*, token::*};

pub fn oauth_service() -> Scope {
    web::scope("/oauth")
        .service(google_oauth_service())
        .service(github_oauth_service())
        .guard(Acceptable::new(mime::APPLICATION_JSON).match_star_star())
        .service(token)
        .service(register)
        .service(revoke)
}

#[derive(OpenApi)]
#[openapi(
    paths(
        token,
        register,
        revoke
    ),
    nest(
        (path = "/google", api = google::GoogleApi),
        (path = "/github", api = github::GithubApi)
    ),
    components(
        schemas(),
        responses()
    )
)]
pub(crate) struct OauthApi;
