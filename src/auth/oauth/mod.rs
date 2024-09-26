pub mod basic;
pub mod google;
pub(crate) mod provider;
pub(crate) mod scopes;
pub mod github;
pub mod error;

use crate::auth::oauth::google::google_oauth_service;
use actix_web::{web, Scope};
use anyhow::Result;
pub use google::GoogleOauth;
use serde::Deserialize;
use utoipa::{IntoParams, OpenApi};
use crate::auth::oauth::github::{github_oauth_service, GithubOauth};

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
    web::scope("/oauth").service(google_oauth_service()).service(github_oauth_service())
}

#[allow(dead_code)]
#[derive(Deserialize, Debug, IntoParams)]
struct OAuthCallbackQuery {
    #[param(example = "code123")]
    code: String,
    #[param(example = "state123")]
    state: String,
}

use google::*;
use github::__path_login as __path_github_login;

#[derive(OpenApi)]
#[openapi(paths(google_login, github_login), components())]
pub struct OauthApi;
