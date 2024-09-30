use crate::auth::oauth::error::OauthError;
use crate::auth::oauth::provider::google::GoogleProvider;
use crate::auth::oauth::scopes::google::{GoogleScope, GoogleScopes};
use crate::auth::oauth::OAuthCallbackQuery;
use crate::auth::{Role, UserInfo};
use crate::models::datetime::Datetime;
use crate::utils::oauth_client::define_oauth_client;
use crate::AppState;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder, Scope};
use anyhow::Result;
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

#[utoipa::path(
    context_path = "/google",
    responses(
        (status = 302, description = "Redirect to Google login page"),
    ),
    tag = "oauth",
)]
#[get("/login")]
pub async fn google_login(state: web::Data<AppState>) -> impl Responder {
    info!("Redirecting to Google login page");
    let oauth = state.oauth.clone();

    let (auth_url, _csrf_token) = oauth.google.get_auth_url();

    HttpResponse::Found()
        .append_header(("Location", auth_url))
        .finish()
}

#[get("/callback")]
pub async fn google_callback(
    query: web::Query<OAuthCallbackQuery>,
    state: web::Data<AppState>,
    req: HttpRequest,
) -> impl Responder {
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

            HttpResponse::Found()
                .append_header(("Location", redirect_url))
                .finish()
        }

        Err(err) => {
            error!("Error exchanging code: {}", err);
            HttpResponse::InternalServerError().body(err.to_string())
        }
    }
}

pub fn google_oauth_service() -> Scope {
    web::scope("/google")
        .service(google_login)
        .service(google_callback)
}
