use actix_web::{web, HttpResponse, Scope};
use helper_macros::generate_endpoint;
use oauth2::{AccessToken, RefreshToken};
use rand::{
    distributions::{Alphanumeric, DistString},
    thread_rng,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// The `TokenResponse` type models the response data
/// of the "/token" endpoint.
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
    method: get;
    path: "/token";
    docs: {
        tag: "token",
        context_path: "/local",
        responses: {
            (status = 200, description = "Local OAuth provider token endpoint")
        }
    }
    {
        Ok(HttpResponse::Ok().json(TokenResponse::new()))
    }
}

pub fn local_oauth_service() -> Scope {
    web::scope("/local").service(local_token)
}
