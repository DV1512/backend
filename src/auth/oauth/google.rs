use actix_identity::Identity;
use crate::auth::oauth::error::OauthError;
use crate::auth::oauth::provider::google::GoogleProvider;
use crate::auth::oauth::scopes::google::{GoogleScope, GoogleScopes};
use crate::auth::oauth::OAuthCallbackQuery;
use crate::auth::{Role, UserInfo};
use crate::error::ServerResponseError;
use crate::models::datetime::Datetime;
use crate::utils::oauth_client::define_oauth_client;
use crate::AppState;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Scope};
use actix_web::cookie::Cookie;
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
        let safe_username = {
            let mut username = user_info.name.clone().to_lowercase();
            username.retain(|c| c.is_ascii());
            username.retain(|c| c.is_alphanumeric() || c == '_');
            username
        };

        Self {
            id: None,
            email: user_info.email,
            url_safe_username: safe_username,
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

generate_endpoint! {
    fn google_login;
    method: get;
    path: "/login";
    docs: {
        context_path: "/google",
        tag: "oauth",
        responses: {
            (status = 302, description = "Redirect to Google login page"),
        }
    }
    params: {
        state: web::Data<AppState>,
    };
    {
        info!("Redirecting to Google login page");
        let oauth = state.oauth.clone();

        let (auth_url, _csrf_token) = oauth.google.get_auth_url();

        Ok(HttpResponse::Found()
            .append_header(("Location", auth_url))
            .finish())
    }
}

generate_endpoint! {
    fn google_callback;
    method: get;
    path: "/callback";
    docs: {
        context_path: "/google",
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
            .google
            .exchange_code(query.code.clone(), &state.db)
            .await
        {
            Ok(session) => {
                let redirect_url = format!("{}users?token={}", frontend_url, session.access_token);
                Identity::login(&req.extensions(), session.id.expect("Failed to get user id").to_string()).unwrap();

                Ok(HttpResponse::Found()
                    .append_header(("Location", redirect_url))
                    .cookie(Cookie::new("token", session.access_token))
                    .finish())
            }

            Err(err) => {
                error!("Error exchanging code: {}", err);
                Err(ServerResponseError::InternalError(err.to_string()))
            }
        }
    }
}

pub fn google_oauth_service() -> Scope {
    web::scope("/google")
        .service(google_login)
        .service(google_callback)
}
