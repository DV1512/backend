pub mod basic;
pub mod error;
pub mod github;
pub mod google;
pub mod local;
pub(crate) mod provider;
pub(crate) mod scopes;

use crate::auth::oauth::github::{github_oauth_service, GithubOauth};
use crate::auth::oauth::google::google_oauth_service;
use crate::auth::oauth::local::local_oauth_service;
use actix_web::{web, Scope};
use anyhow::Result;
pub use google::GoogleOauth;
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
        .service(local_oauth_service())
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

#[derive(OpenApi)]
#[openapi(paths(google_login, github_login), components())]
pub struct OauthApi;
