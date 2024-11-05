use crate::auth::session::UserSession;
use actix_identity::Identity;
use actix_web::{dev::Payload, error::ErrorUnauthorized, Either, FromRequest, HttpRequest, Result};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use std::future::Future;
use std::pin::Pin;

/// This Extractor is used to get the token from the request, this does not check if the token is valid.
pub(crate) type Token = Either<Identity, BearerAuth>;

/// This Extractor is only used to make sure that the user has a valid session but does not need to use the session or token
pub(crate) struct Authenticated;

/// This Extractor is used to get the token from the request and check if the token is valid.
pub(crate) struct AuthenticatedToken(pub(crate) String);

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

impl IntoSession for Token {
    #[inline]
    fn get_token(&self) -> String {
        match self {
            Either::Left(identity) => identity.id().unwrap_or("".to_string()),
            Either::Right(session) => session.get_token(),
        }
    }
}

impl IntoSession for Identity {
    #[inline]
    fn get_token(&self) -> String {
        self.id().unwrap_or("".to_string())
    }
}

impl IntoSession for BearerAuth {
    #[inline]
    fn get_token(&self) -> String {
        self.token().to_string()
    }
}

impl IntoSession for AuthenticatedToken {
    #[inline]
    fn get_token(&self) -> String {
        self.0.clone()
    }
}

impl FromRequest for AuthenticatedToken {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let token = UserSession::from_request(req, payload);

        Box::pin(async move {
            let token = token.await;
            match token {
                Ok(session) => {
                    let token = session.access_token;

                    Ok(AuthenticatedToken(token))
                }
                Err(_) => Err(ErrorUnauthorized("Unauthorized")),
            }
        })
    }
}

impl FromRequest for Authenticated {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let session = UserSession::from_request(req, payload);

        Box::pin(async move {
            let session = session.await;
            match session {
                Ok(_) => Ok(Authenticated),
                Err(_) => Err(ErrorUnauthorized("Unauthorized")),
            }
        })
    }
}

impl FromRequest for UserSession {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let token = Token::from_request(req, payload);

        Box::pin(async {
            let token = token.await;

            match token {
                Ok(token) => match token.get_session().await {
                    Some(session) => Ok(session),
                    None => Err(ErrorUnauthorized("Unauthorized")),
                },
                Err(_err) => Err(ErrorUnauthorized("Unauthorized")),
            }
        })
    }
}
