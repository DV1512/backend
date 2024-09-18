use crate::auth::oauth::provider::{OauthProvider, OauthProviderName};
use crate::{PaginationResponse, Record, INTERNAL_DB};
use anyhow::bail;
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};

mod oauth;
pub mod session;
pub mod users;

#[derive(Debug, Serialize, Deserialize, PartialOrd, Eq, PartialEq, Clone, Default)]
pub(crate) enum Role {
    Owner,
    Admin,
    #[default]
    User,
}

#[derive(Debug, Serialize, Deserialize, PartialOrd, Eq, PartialEq, Clone)]
pub(crate) struct UserInfo {
    pub(crate) id: Option<Thing>,
    pub(crate) email: String,
    pub(crate) url_safe_username: String,
    pub(crate) username: String,
    pub(crate) first_name: String,
    pub(crate) last_name: String,
    pub(crate) created_at: Datetime,
    pub(crate) last_login: Option<Datetime>,
    pub(crate) picture: Option<String>,
    pub(crate) role: Role,
}

#[derive(Debug, Serialize, Deserialize, PartialOrd, Eq, PartialEq, Clone)]
pub(crate) struct Users {
    pub(crate) users: Vec<UserInfo>,

    #[serde(flatten)]
    pub(crate) pagination: PaginationResponse,
}

async fn create_auth_for_user(
    user_id: Record,
    providers: Vec<OauthProvider>,
    password: Option<String>,
) -> anyhow::Result<()> {
    let mut require_password = false;
    let providers_ids = providers
        .iter()
        .map(|p| p.id.clone())
        .collect::<Vec<Thing>>();
    let user_id = user_id.id;

    for provider in providers {
        if provider.name == OauthProviderName::Email {
            require_password = true;
        }
    }

    if require_password && password.is_none() {
        bail!("User requires password. Please provide one.");
    }

    let sql = "CREATE user_auth set providers = $providers, password = $password";

    let mut res = INTERNAL_DB
        .query(sql)
        .bind(("providers", providers_ids))
        .bind(("password", password))
        .await?;

    let user_auth: Option<AuthForRelation> = res.take(0)?;

    if let Some(user_auth) = user_auth {
        let query = "RELATE $user_auth->auth_for->$user_id";
        INTERNAL_DB
            .query(query)
            .bind(("user_auth", user_auth.id))
            .bind(("user_id", user_id))
            .await?;
    } else {
        bail!("Error creating user_auth");
    }

    Ok(())
}

#[derive(Debug, Serialize, Deserialize, PartialOrd, Eq, PartialEq, Clone)]
pub struct AuthForRelation {
    id: Thing,
    created_at: Datetime,
    updated_at: Datetime,
    password: Option<String>,
    providers: Vec<Thing>,
}
