mod github;
pub mod google;

use crate::INTERNAL_DB;
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use surrealdb::sql::{Id, Thing};
use tracing::warn;

#[derive(Debug, Clone, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum OauthProviderName {
    Email,
    Google,
    Github,
}

impl From<OauthProviderName> for String {
    fn from(v: OauthProviderName) -> Self {
        match v {
            OauthProviderName::Email => "Basic".to_owned(),
            OauthProviderName::Google => "Google".to_owned(),
            OauthProviderName::Github => "GitHub".to_owned(),
        }
    }
}

impl From<OauthProviderName> for Id {
    fn from(v: OauthProviderName) -> Self {
        Id::String(v.into())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct OauthProvider {
    pub(crate) id: Thing,
    pub(crate) name: OauthProviderName,
    pub(crate) config: Option<OauthProviderConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub(crate) struct OauthProviderConfig {
    pub(crate) auth_url: Option<String>,
    pub(crate) token_url: Option<String>,
    pub(crate) scopes: Option<Vec<String>>,
    pub(crate) user_info_url: Option<String>,
    pub(crate) redirect_endpoint: Option<String>,
    pub(crate) additional_config: BTreeMap<String, String>,
}

impl OauthProviderConfig {
    pub fn get_auth_url(&self) -> String {
        if let Some(auth_url) = &self.auth_url {
            auth_url.clone()
        } else {
            warn!("Provider config does not have an auth_url. Please add it to the database.");
            String::new()
        }
    }

    pub fn get_token_url(&self) -> String {
        if let Some(token_url) = &self.token_url {
            token_url.clone()
        } else {
            warn!("Provider config does not have a token_url. Please add it to the database.");
            String::new()
        }
    }

    pub fn get_scopes(&self) -> Vec<String> {
        if let Some(scopes) = &self.scopes {
            scopes.clone()
        } else {
            warn!("Provider config does not have scopes. Please add it to the database.");
            Vec::new()
        }
    }

    pub fn get_user_info_url(&self) -> String {
        if let Some(user_info_url) = &self.user_info_url {
            user_info_url.clone()
        } else {
            warn!("Provider config does not have a user_info_url. Please add it to the database.");
            String::new()
        }
    }

    pub fn get_redirect_endpoint(&self) -> String {
        if let Some(redirect_endpoint) = &self.redirect_endpoint {
            redirect_endpoint.clone()
        } else {
            warn!(
                "Provider config does not have a redirect_endpoint. Please add it to the database."
            );
            String::new()
        }
    }

    #[allow(dead_code)]
    pub fn get_additional_config(&self, key: String) -> String {
        if let Some(config) = self.additional_config.get(&key) {
            config.clone()
        } else {
            warn!("Provider config does not have an additional_config with key: {:?}. Please add it to the database.", key);
            String::new()
        }
    }
}

impl OauthProvider {
    pub async fn fetch_provider(name: OauthProviderName) -> Result<Self> {
        let mut res = INTERNAL_DB
            .query("SELECT *, config.* from provider WHERE name = $name;")
            .bind(("name", name.clone()))
            .await?;

        let provider: Option<Self> = res.take(0)?;

        if let Some(provider) = provider {
            Ok(provider)
        } else {
            bail!("Provider not found for name: {:?}", name)
        }
    }
}

pub trait Provider
where
    Self: From<OauthProvider> + Into<OauthProvider>,
{
    const NAME: OauthProviderName;

    fn get_config(&mut self) -> OauthProviderConfig;

    async fn fetch() -> Result<Self> {
        Ok(OauthProvider::fetch_provider(Self::NAME).await?.into())
    }
}
