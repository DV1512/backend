use super::provider::github::GithubProvider;
use super::scopes::github::{GithubScope, GithubScopes};
use crate::auth::oauth::error::OauthError;
use crate::models::datetime::Datetime;
use crate::models::user_info::{Role, UserInfo};
use crate::utils::oauth_client::define_oauth_client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
struct GithubUserInfo {
    login: String,
    id: i32,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    #[serde(rename = "type")]
    root_type: String,
    site_admin: bool,
    name: Option<String>,
    company: Option<String>,
    blog: String,
    location: String,
    email: Option<String>,
    hireable: bool,
    bio: Option<String>,
    twitter_username: Option<String>,
    notification_email: Option<String>,
    public_repos: i32,
    public_gists: i32,
    followers: i32,
    following: i32,
    created_at: String,
    updated_at: String,
}

define_oauth_client!(
    GithubOauth,
    GithubProvider,
    GithubScopes,
    GithubScope,
    GithubUserInfo,
    {
        client_id_env: "GITHUB_CLIENT_ID",
        client_secret_env: "GITHUB_CLIENT_SECRET",
        base_url_env: "BASE_URL",
        default_base_url: "http://localhost:9999",
        user_info_mapping: |github_user_info| {
            Ok::<UserInfo, OauthError>(UserInfo {
                id: None,
                email: github_user_info.email.unwrap_or_default(),
                url_safe_username: None,
                username: github_user_info.name.unwrap_or_else(|| github_user_info.login.clone()),
                first_name: "".to_string(),
                last_name: "".to_string(),
                created_at: Datetime::default(),
                last_login: None,
                picture: Some(github_user_info.avatar_url),
                role: Role::default(),
            })
        },
    }
);
