pub mod basic;
pub mod error;
pub mod github;
pub mod google;
pub mod local;
pub mod logout;
pub(crate) mod provider;
pub mod register;
pub(crate) mod scopes;

use crate::auth::oauth::github::{github_oauth_service, GithubOauth};
use crate::auth::oauth::google::{google_oauth_service, GoogleOauth};
use crate::auth::oauth::local::token;
use crate::auth::oauth::register::register_endpoint;
use actix_web::guard::Acceptable;
use actix_web::{web, Scope};
use anyhow::Result;
use logout::logout as logout_endpoint;
use serde::Deserialize;
use utoipa::{IntoParams, OpenApi};

#[derive(Debug, Clone)]
pub struct Oauth {
    pub google: GoogleOauth,
    pub github: GithubOauth,
}

impl Oauth {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            google: GoogleOauth::new().await?,
            github: GithubOauth::new().await?,
        })
    }
}

pub fn oauth_service() -> Scope {
    web::scope("/oauth")
        .service(google_oauth_service())
        .service(github_oauth_service())
        .service(logout_endpoint)
        .guard(Acceptable::new(mime::APPLICATION_JSON).match_star_star())
        .service(token)
        .service(register_endpoint)
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, IntoParams)]
struct OAuthCallbackQuery {
    #[param(example = "code123")]
    code: String,
    #[param(example = "state123")]
    state: String,
}

use github::__path_login as __path_github_login;
use google::*;

use crate::models::refresh_token::RefreshToken;
use local::{TokenRequest,  TokenResponseExample,  __path_token};
use logout::__path_logout;

#[derive(OpenApi)]
#[openapi(
    paths(google_login, github_login, token, logout),
    components(schemas(TokenRequest, RefreshToken), responses(TokenResponseExample))
)]
pub struct OauthApi;
