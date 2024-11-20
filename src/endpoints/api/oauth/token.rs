use actix_identity::Identity;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use actix_web::http::header;
use actix_web::http::header::CacheDirective;
use tracing::info;
use helper_macros::generate_endpoint;
use crate::dto::{TokenRequest, TokenResponse, TokenResponseExample};
use crate::auth::session::UserSession;
use crate::error::ServerResponseError;
use crate::services::token::validate_user;
use crate::state::AppState;

generate_endpoint! {
    fn token;
    method: post;
    path: "/token";
    docs: {
        tag: "oauth",
        responses: {
            (status = 200, response = TokenResponseExample),
            (status = 404, description = "User not found or invalid credentials"),
            (status = 501, description = "Refresh token grant type is not implemented yet")
        }
    }
    params: {
        req: HttpRequest,
        state: web::Data<AppState>,
        data: web::Form<TokenRequest>,
    };
    {
        info!("Requesting access token");
        let db = state.db.clone();
        match data.0 {
            TokenRequest::RefreshToken { refresh_token: _ } => Err(ServerResponseError::NotImplementedWithMessage("Refreshing tokens not yet supported".to_string())),
            TokenRequest::Password { username, password } => {
                let user = validate_user(username, password, &db).await?;
                let response = TokenResponse::new();
                let token = response.access_token.secret().to_string();

                let session = UserSession::new(token.clone(), Some(response.refresh_token.secret().to_string()), user.email, user.id);
                Identity::login(&req.extensions(), token).unwrap();
                session.create().await?;

                Ok(HttpResponse::Ok()
                    .insert_header(header::CacheControl(vec![
                        CacheDirective::NoCache,
                        CacheDirective::NoStore,
                    ]))
                    .json(response))
            }
        }
    }
}
