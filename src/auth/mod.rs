use crate::auth::oauth::provider::{OauthProvider, OauthProviderName};
use crate::server::db::INTERNAL_DB;
use anyhow::bail;
use serde::{Deserialize, Serialize};
//use surrealdb::sql::{Datetime, Thing};
use crate::models::datetime::Datetime;
use crate::models::thing::Thing;
use std::string::String;
//use surrealdb::sql::Thing;
use utoipa::{ToResponse, ToSchema};
use crate::dto::PaginationResponse;
use crate::models::Record;

pub mod oauth;
pub mod session;
pub mod users;

#[derive(Debug, Serialize, Deserialize, PartialOrd, Eq, PartialEq, Clone, Default, ToSchema)]
pub enum Role {
    Owner,
    Admin,
    #[default]
    User,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, ToSchema, PartialOrd, Eq, PartialEq)]
pub struct UserInfo {
    #[schema(example = "user:123456")]
    pub id: Option<Thing>,
    #[schema(example = "johndoe@example.com")]
    pub email: String,
    #[schema(example = "johndoe")]
    pub url_safe_username: String,
    #[schema(example = "John Doe")]
    pub username: String,
    #[schema(example = "John")]
    pub first_name: String,
    #[schema(example = "Doe")]
    pub last_name: String,
    #[schema(example = "2021-09-15T14:28:23Z")]
    pub created_at: Datetime,
    #[schema(example = "2021-09-15T14:28:23Z")]
    pub last_login: Option<Datetime>,
    #[schema(example = "https://example.com/avatar.jpg")]
    pub picture: Option<String>,
    pub role: Role,
}

#[derive(ToResponse)]
#[allow(dead_code)]
pub enum UserInfoExampleResponses {
    #[response(examples(
        ("JohnDoe" = (value = json!({
            "id": {
                "id": "5f4d0c8f-1b78-4e3f-9d0c-0b0d0b0b0b0b",
                "tb": "user",
            },
            "email": "johndoe@example.com",
            "url_safe_username": "johndoe",
            "username": "John Doe",
            "first_name": "John",
            "last_name": "Doe",
            "created_at": "2021-09-15T14:28:23Z",
            "last_login": "2021-09-15T14:28:23Z",
            "picture": "https://example.com/avatar.jpg",
            "role": "Owner",
         }
         )))
    ))]
    User(#[content("application/json")] UserInfo),
}

#[derive(Debug, Serialize, Deserialize, PartialOrd, Eq, PartialEq, Clone)]
pub(crate) struct Users {
    pub(crate) users: Vec<UserInfo>,

    #[serde(flatten)]
    pub(crate) pagination: PaginationResponse,
}

#[tracing::instrument(skip(password))]
pub(crate) async fn create_auth_for_user(
    user_id: Record,
    providers: Vec<OauthProvider>,
    password: Option<String>,
) -> anyhow::Result<()> {
    let require_password = providers.iter().any(|p| p.name == OauthProviderName::Email);
    let providers_ids = providers.iter().map(|p| p.id.clone()).collect::<Vec<_>>();
    let user_id = user_id.id;

    if require_password && password.is_none() {
        bail!("User requires password. Please provide one.");
    }

    let mut res = if password.is_some() {
        let sql = "CREATE user_auth set providers = $providers, password = $password";

        INTERNAL_DB
            .query(sql)
            .bind(("providers", providers_ids))
            .bind(("password", password))
            .await?
    } else {
        let sql = "CREATE user_auth set providers = $providers";

        INTERNAL_DB
            .query(sql)
            .bind(("providers", providers_ids))
            .await?
    };

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
