use crate::auth::oauth::provider::{
    OauthProvider, OauthProviderConfig, OauthProviderName, Provider,
};
use crate::auth::oauth::scopes::github::GithubScopes;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::env;
use surrealdb::sql::Thing;
use tracing::warn;

#[derive(Debug, Clone, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct GithubProvider {
    pub(crate) id: Thing,
    pub(crate) name: OauthProviderName,
    pub(crate) config: Option<OauthProviderConfig>,
}
impl From<OauthProvider> for GithubProvider {
    fn from(v: OauthProvider) -> Self {
        Self {
            id: v.id,
            name: v.name,
            config: v.config,
        }
    }
}

impl From<GithubProvider> for OauthProvider {
    fn from(v: GithubProvider) -> Self {
        Self {
            id: v.id,
            name: v.name,
            config: v.config,
        }
    }
}

impl Provider for GithubProvider {
    const NAME: OauthProviderName = OauthProviderName::Github;
    fn get_config(&mut self) -> OauthProviderConfig {
        if let Some(config) = &self.config {
            config.clone()
        } else {
            warn!("There is no github configured yet");

            let github_auth_url = env::var("GITHUB_AUTH_URL")
                .unwrap_or("https://github.com/login/oauth/authorize ".to_string());
            let github_token_url = env::var("GITHUB_TOKEN_URL")
                .unwrap_or("https://github.com/login/oauth/access_token".to_string());

            let scopes: GithubScopes = GithubScopes::default();

            let user_info_url = env::var("GITHUB_USER_INFO_URL")
                .unwrap_or("https://api.github.com/user_info".to_string());

            let redirect_endpoint = "/oauth/github/callback".to_string();

            let config = OauthProviderConfig {
                auth_url: Some(github_auth_url),
                token_url: Some(github_token_url),
                scopes: Some(scopes.into()),
                user_info_url: Some(user_info_url),
                redirect_endpoint: Some(redirect_endpoint),
                additional_config: BTreeMap::new(),
            };

            self.config = Some(config.clone());

            config
        }
    }
}
