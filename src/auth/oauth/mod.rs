pub mod basic;
pub mod error;
pub mod github;
pub mod google;
pub mod local;
pub mod logout;
pub(crate) mod provider;
pub mod register;
pub(crate) mod scopes;
pub mod update;

use crate::auth::oauth::github::GithubOauth;
use crate::auth::oauth::google::GoogleOauth;
use crate::auth::oauth::local::token;
use crate::dto::UserRegistrationRequest;
use actix_web::guard::Acceptable;
use actix_web::{web, Scope};
use anyhow::Result;

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
