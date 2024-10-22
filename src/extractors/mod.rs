use crate::auth::session::UserSession;
use actix_identity::Identity;
use actix_web::{dev::Payload, error::ErrorUnauthorized, Either, FromRequest, HttpRequest, Result};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use std::future::Future;
use std::pin::Pin;
use tosic_utils::wrap_external_type;
use tracing::warn;

pub(crate) type Auth = Either<Identity, Bearer>;

wrap_external_type! {
    pub(crate) struct Bearer(BearerAuth);
}

impl FromRequest for Bearer {
    type Error = actix_web::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let token_result = BearerAuth::from_request(req, payload).into_inner();

        match token_result {
            Ok(token) => Box::pin(async move {
                if UserSession::fetch_by_access_token(token.token().to_string()).await.is_none() {
                    warn!("Unauthorized access token: {}", token.token());
                    return Err(ErrorUnauthorized("Unauthorized"));
                }

                Ok(Bearer(token))
            }),
            Err(_err) => Box::pin(async { Err(ErrorUnauthorized("Unauthorized")) }),
        }
    }
}
