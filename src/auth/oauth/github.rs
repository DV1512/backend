use super::provider::github::GithubProvider;
use super::scopes::github::{GithubScope, GithubScopes};
use crate::auth::oauth::error::OauthError;
use crate::auth::oauth::OAuthCallbackQuery;
use crate::auth::{Role, UserInfo};
use crate::models::datetime::Datetime;
use crate::state::AppState;
use crate::utils::oauth_client::define_oauth_client;
use actix_web::{web, HttpRequest, HttpResponse, Scope};
use serde::{Deserialize, Serialize};
use tracing::{error, info};
use helper_macros::generate_endpoint;
use crate::error::ServerResponseError;

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
            let safe_username = {
                let mut username = github_user_info.login.clone().to_lowercase();
                username.retain(|c| c.is_ascii_alphanumeric() || c == '_');
                username
            };

            Ok::<UserInfo, OauthError>(UserInfo {
                id: None,
                email: github_user_info.email.unwrap_or_default(),
                url_safe_username: safe_username,
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

generate_endpoint! {
    fn login;
    method: get;
    path: "/login";
    docs: {
        context_path: "/github",
        tag: "oauth",
        responses: {
            (status = 302, description = "Redirect to Github login page"),
        }
    }
    params: {
        state: web::Data<AppState>,
    };
    {
        info!("Redirecting to Github login page");
        let oauth = state.oauth.clone();

        let (auth_url, _csrf_token) = oauth.github.get_auth_url();

        Ok(HttpResponse::Found()
            .append_header(("Location", auth_url))
            .finish())
    }
}

generate_endpoint! {
    fn callback;
    method: get;
    path: "/callback";
    docs: {
        context_path: "/github",
        tag: "oauth",
        responses: {
            (status = 302, description = "Redirect to frontend"),
        }
    }
    params: {
        state: web::Data<AppState>,
        query: web::Query<OAuthCallbackQuery>,
        req: HttpRequest,
    };
    {
        info!("Google callback received");

        let oauth = state.oauth.clone();

        let frontend_url = req.url_for_static("frontend").unwrap().to_string();

        match oauth
            .github
            .exchange_code(query.code.clone(), &state.db)
            .await
        {
            Ok(session) => {
                let redirect_url = format!("{}users?token={}", frontend_url, session.access_token);

                Ok(HttpResponse::Found()
                    .append_header(("Location", redirect_url))
                    .finish())
            }

            Err(err) => {
                error!("Error exchanging code: {}", err);
                Err(ServerResponseError::InternalError(err.to_string()))
            }
        }
    }
}

pub fn github_oauth_service() -> Scope {
    web::scope("/github").service(login).service(callback)
}
