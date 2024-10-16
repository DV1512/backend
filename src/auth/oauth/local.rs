use crate::auth::oauth::provider::OauthProvider;
use crate::error::ServerResponseError;
use crate::state::AppState;
use actix_web::http::header::CacheDirective;
use actix_web::{http::header, web, HttpResponse, Scope};
use helper_macros::generate_endpoint;
use oauth2::{AccessToken, RefreshToken};
use rand::{
    distributions::{Alphanumeric, DistString},
    thread_rng,
};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};
use tracing::info;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(tag = "grant_type", rename_all = "snake_case")]
enum TokenRequest {
    Password { username: String, password: String },
    RefreshToken { refresh_token: RefreshToken },
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct TokenResponse<'a> {
    access_token: AccessToken,
    refresh_token: RefreshToken,
    token_type: &'a str,
    expires_in: i64,
}

/// Returns a random alphanumeric string of length `length`.
fn random_string(length: usize) -> String {
    Alphanumeric.sample_string(&mut thread_rng(), length)
}

impl<'a> TokenResponse<'a> {
    fn new() -> Self {
        Self {
            access_token: AccessToken::new(random_string(16)),
            refresh_token: RefreshToken::new(random_string(16)),
            token_type: "bearer",
            expires_in: 3600,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Foo {
    id: Option<Thing>,
    providers: Option<Vec<OauthProvider>>,
    created_at: Datetime,
    updated_at: Datetime,
    password: Option<String>,
}

generate_endpoint! {
    fn local_token;
    method: post;
    path: "/token";
    docs: {
        tag: "token",
        context_path: "/local",
        responses: {
            (status = 200, description = "Local OAuth provider token endpoint")
        }
    }
    params: {
        state: web::Data<AppState>,
        data: web::Form<TokenRequest>,
    };
    {
        info!("Recieved token request: {:?}", &data);

        match data.into_inner() {
            TokenRequest::RefreshToken { refresh_token } => Err(ServerResponseError::BadRequest(
                "Refreshing tokens not yet supported".to_string(),
            )),
            TokenRequest::Password { username, password } => {
                let query =
                    r#"SELECT COUNT() FROM user WHERE username = $username AND password = $password;"#;
                let query_result: Option<i32> = state
                    .db
                    .query(query)
                    .bind(("username", username))
                    .bind(("password", password))
                    .await?
                    .take("count")?;

                let valid_user = query_result.is_some_and(|count| count > 0);
                if valid_user {
                    Ok(HttpResponse::Ok()
                        .insert_header(header::CacheControl(vec![
                            CacheDirective::NoCache,
                            CacheDirective::NoStore,
                        ]))
                        .json(TokenResponse::new()))
                } else {
                    Err(ServerResponseError::NotFound)
                }
            }
        }
    }
}

pub fn local_oauth_service() -> Scope {
    web::scope("/local").service(local_token)
}
