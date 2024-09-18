use crate::auth::oauth::basic::Scopes;
use crate::auth::oauth::provider::{
    OauthProvider, OauthProviderConfig, OauthProviderName, Provider,
};
use crate::auth::oauth::scopes::google::{GoogleScope, GoogleScopes};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::env;
use surrealdb::sql::Thing;
use tracing::warn;

#[derive(Debug, Clone, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct GoogleProvider {
    pub(crate) id: Thing,
    pub(crate) name: OauthProviderName,
    pub(crate) config: Option<OauthProviderConfig>,
}

impl From<OauthProvider> for GoogleProvider {
    fn from(v: OauthProvider) -> Self {
        Self {
            id: v.id,
            name: v.name,
            config: v.config,
        }
    }
}

impl From<GoogleProvider> for OauthProvider {
    fn from(v: GoogleProvider) -> Self {
        Self {
            id: v.id,
            name: v.name,
            config: v.config,
        }
    }
}

impl Provider for GoogleProvider {
    const NAME: OauthProviderName = OauthProviderName::Google;

    fn get_config(&mut self) -> OauthProviderConfig {
        if let Some(config) = &self.config {
            config.clone()
        } else {
            warn!(
                "Provider config not found for name: {:?}. Please add it to the database.",
                Self::NAME
            );
            let google_auth_url = env::var("GOOGLE_AUTH_URL")
                .unwrap_or("https://accounts.google.com/o/oauth2/auth".to_string());
            let google_token_url = env::var("GOOGLE_TOKEN_URL")
                .unwrap_or("https://oauth2.googleapis.com/token".to_string());
            let scopes = GoogleScopes::default()
                .add_scope(GoogleScope::Email)
                .add_scope(GoogleScope::Profile)
                .add_scope(GoogleScope::OpenId);
            let user_info_url = env::var("GOOGLE_USER_INFO_URL")
                .unwrap_or("https://www.googleapis.com/oauth2/v3/userinfo".to_string());

            let redirect_endpoint = "/oauth/google/callback".to_string();

            let config = OauthProviderConfig {
                auth_url: Some(google_auth_url),
                token_url: Some(google_token_url),
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
