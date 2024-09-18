pub mod basic;
pub mod google;
pub(crate) mod provider;
mod scopes;

use crate::auth::oauth::google::google_oauth_service;
use actix_web::{web, Scope};
use anyhow::Result;
pub use google::GoogleOauth;
use serde::Deserialize;

pub struct Oauth {
    pub google: GoogleOauth,
}

impl Oauth {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            google: GoogleOauth::new().await?,
        })
    }
}

pub fn oauth_service() -> Scope {
    web::scope("/oauth").service(google_oauth_service())
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct OAuthCallbackQuery {
    code: String,
    state: String,
}
