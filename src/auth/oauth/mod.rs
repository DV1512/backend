pub mod basic;
pub mod error;
pub mod github;
pub mod google;
pub(crate) mod provider;
pub(crate) mod scopes;

use crate::auth::oauth::github::GithubOauth;
use crate::auth::oauth::google::GoogleOauth;
use anyhow::Result;

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
