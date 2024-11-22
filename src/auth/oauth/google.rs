use crate::auth::oauth::error::OauthError;
use crate::auth::oauth::provider::google::GoogleProvider;
use crate::auth::oauth::scopes::google::{GoogleScope, GoogleScopes};

use crate::auth::{Role, UserInfo};
use crate::error::ServerResponseError;
use crate::models::datetime::Datetime;
use crate::utils::oauth_client::define_oauth_client;
use crate::AppState;
use actix_identity::Identity;
use actix_web::cookie::Cookie;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Scope};
use anyhow::Result;
use helper_macros::generate_endpoint;
use serde::{Deserialize, Serialize};
use tracing::{error, info};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct GoogleUserInfo {
    pub sub: String,
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
    pub email: String,
    pub email_verified: bool,
}

impl From<GoogleUserInfo> for UserInfo {
    fn from(user_info: GoogleUserInfo) -> Self {
        Self {
            id: None,
            email: user_info.email,
            url_safe_username: None,
            username: user_info.name,
            first_name: user_info.given_name,
            last_name: user_info.family_name,
            created_at: Datetime::default(),
            last_login: None,
            picture: Some(user_info.picture),
            role: Role::default(),
        }
    }
}

define_oauth_client! {
    GoogleOauth,
    GoogleProvider,
    GoogleScopes,
    GoogleScope,
    GoogleUserInfo,
    {
        client_id_env: "GOOGLE_CLIENT_ID",
        client_secret_env: "GOOGLE_CLIENT_SECRET",
        base_url_env: "BASE_URL",
        default_base_url: "http://localhost:9999",
        user_info_mapping: |google_user_info| {
            Ok::<UserInfo, OauthError>(UserInfo::from(google_user_info))
        },
    }
}
