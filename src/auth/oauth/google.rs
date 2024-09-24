use super::basic::BasicOauth;
use crate::auth::oauth::provider::google::GoogleProvider;
use crate::auth::oauth::provider::Provider;
use crate::auth::oauth::scopes::google::{GoogleScope, GoogleScopes};
use crate::auth::oauth::OAuthCallbackQuery;
use crate::auth::users::create::create_user;
use crate::auth::users::get_user;
use crate::auth::{create_auth_for_user, session::UserSession, Role, UserInfo};
use crate::{AppState, Record};
use actix_web::{get, web, HttpRequest, HttpResponse, Responder, Scope};
use anyhow::{bail, Result};
use api_forge::{ApiRequest, Request};
use oauth2::TokenResponse;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;
use surrealdb::sql::Datetime;
use surrealdb::{Connection, Surreal};
use tracing::{debug, error, info};

#[derive(Debug, Default, Clone, Request, Serialize)]
#[request(endpoint = "", response_type = GoogleUserInfo, authentication = Bearer)] // the endpoint is ignored since we will provide the full url when sending the request
struct GetGoogleUserInfo;

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

#[derive(Debug, Clone)]
pub struct GoogleOauth {
    basic: BasicOauth<GoogleScopes, GoogleScope>,
    details: GoogleProvider,
}

impl GoogleOauth {
    #[tracing::instrument(name = "GoogleOauth::new")]
    pub async fn new() -> Result<Self> {
        let mut details = GoogleProvider::fetch().await?;

        let config = details.get_config();

        if config.auth_url.is_none()
            || config.token_url.is_none()
            || config.scopes.is_none()
            || config.user_info_url.is_none()
        {
            bail!("Google Provider is not configured correctly");
        }

        let base_url = env::var("BASE_URL").unwrap_or("http://localhost:9999".to_string());
        let endpoint = config.get_redirect_endpoint();
        let redirect_url = format!("{base_url}{endpoint}");

        let client_id = env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set");
        let client_secret =
            env::var("GOOGLE_CLIENT_SECRET").expect("GOOGLE_CLIENT_SECRET must be set");

        let basic = BasicOauth::new(
            client_id,
            client_secret,
            redirect_url,
            config.get_auth_url(),
            config.get_token_url(),
            config.get_scopes().into(),
        )?;

        Ok(Self { basic, details })
    }

    pub fn get_auth_url(&self) -> (String, oauth2::CsrfToken) {
        self.basic.get_authorization_url()
    }

    #[tracing::instrument(skip(self, code, db))]
    async fn exchange_code_internal<T>(
        &self,
        code: String,
        db: &Arc<Surreal<T>>,
    ) -> Result<UserSession>
    where
        T: Connection,
    {
        let token = match self.basic.exchange_code_for_token(code).await {
            Ok(t) => t,
            Err(e) => {
                error!("Failed to exchange code for token: {:?}", e);
                bail!(e)
            }
        };

        let user_info = match self.get_user_info(token.access_token().secret()).await {
            Ok(u) => u,
            Err(e) => {
                error!("Failed to fetch user info: {:?}", e);
                bail!(e)
            }
        };

        // TODO: Extract this into a method in the UserInfo struct to avoid repetition
        let user: UserInfo = user_info.into();

        let db_user = get_user(db, &user.email).await;

        let record = match db_user {
            None => {
                // Step 5: Create the user if they don't exist
                match create_user(db, user.clone()).await {
                    Ok(new_user_record) => {
                        create_auth_for_user(
                            new_user_record.clone(),
                            vec![self.details.clone().into()],
                            None,
                        )
                        .await
                        .expect("Failed to create auth information for user");
                        new_user_record
                    }
                    Err(e) => {
                        error!("Failed to create user: {:?}", e);
                        bail!(e)
                    }
                }
            }
            Some(existing_user) => {
                if let Some(id) = existing_user.id {
                    Record { id }
                } else {
                    bail!("No ID found for user");
                }
            }
        };

        let access_token = token.access_token().secret().to_string();
        let refresh_token = token.refresh_token().map(|t| t.secret().to_string());

        // If there is no existing session, create a new one else update the existing one
        let session = match UserSession::fetch_by_email(user.email.clone()).await {
            Some(mut s) => {
                s.access_token = access_token;
                s.refresh_token = refresh_token;

                s.update().await?
            }
            None => {
                UserSession::new(access_token, refresh_token, user.email, record.id)
                    .create()
                    .await?
            }
        };

        Ok(session)
    }

    pub async fn exchange_code<C: Connection>(
        &self,
        code: String,
        db: &Arc<Surreal<C>>,
    ) -> Result<UserSession> {
        self.exchange_code_internal(code, db).await
    }

    #[tracing::instrument(skip(self))]
    async fn get_user_info(&self, access_token: &str) -> Result<GoogleUserInfo> {
        debug!("Fetching user info for access token: {}", access_token);

        let user_info_url = self.details.config.clone().unwrap().get_user_info_url();

        if user_info_url.is_empty() {
            error!("User info URL is empty");
            bail!("User info URL is empty");
        }

        /* Ok(GetGoogleUserInfo::default().send_and_parse(&user_info_url, None, Some((access_token.to_string(), None))).await?) */

        let req_builder = GetGoogleUserInfo;

        let response = req_builder
            .send_request(&user_info_url, None, Some((access_token.to_owned(), None)))
            .await?;

        debug!("User info response: {:?}", response);

        Ok(response.json::<GoogleUserInfo>().await?)
    }
}

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
