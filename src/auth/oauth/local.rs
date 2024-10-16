use crate::auth::users::get::get_user_by_username;
use crate::state::AppState;
use actix_web::{web, HttpResponse, Scope};
use helper_macros::generate_endpoint;
use oauth2::{AccessToken, RefreshToken};
use rand::{
    distributions::{Alphanumeric, DistString},
    thread_rng,
};
use serde::{Deserialize, Serialize};
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
        info!("Recieved token request: {:?}", data);
        Ok(HttpResponse::Ok().json(TokenResponse::new()))
    }
}

pub fn local_oauth_service() -> Scope {
    web::scope("/local").service(local_token)
}
