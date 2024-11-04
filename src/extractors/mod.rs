use crate::auth::session::UserSession;
use actix_identity::Identity;
use actix_web::{dev::Payload, error::ErrorUnauthorized, Either, FromRequest, HttpRequest, Result};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use std::future::Future;
use std::pin::Pin;
use tracing::warn;

pub(crate) type Auth = Either<Identity, UserSession>;

pub(crate) trait IntoSession {
    fn get_token(&self) -> String;

    #[inline]
    async fn get_session(&self) -> Option<UserSession> {
        let token = self.get_token();

        if token.is_empty() {
            return None;
        }

        UserSession::fetch_by_access_token(token).await
    }
}

impl IntoSession for Identity {
    #[inline(always)]
    fn get_token(&self) -> String {
        self.id().unwrap_or("".to_string())
    }
}

impl IntoSession for BearerAuth {
    #[inline(always)]
    fn get_token(&self) -> String {
        self.token().to_string()
    }
}

impl FromRequest for UserSession {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let token_result = BearerAuth::from_request(req, payload).into_inner();

        match token_result {
            Ok(token) => Box::pin(async move {
                let session = token.get_session().await;

                if session.is_none() {
                    warn!("Unauthorized access token: {}", token.token());
                    return Err(ErrorUnauthorized("Unauthorized"));
                }

                Ok(session.unwrap())
            }),
            Err(_err) => Box::pin(async { Err(ErrorUnauthorized("Unauthorized")) }),
        }
    }
}
