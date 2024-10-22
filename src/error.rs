#![allow(dead_code)]

use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use actix_web_httpauth::headers::www_authenticate::bearer;

#[derive(Debug, thiserror::Error)]
pub enum ServerResponseError {
    #[error(transparent)]
    AuthorizationError(#[from] actix_web_httpauth::extractors::AuthenticationError<bearer::Bearer>),
    #[error("Database error: {0}")]
    DatabaseError(#[from] surrealdb::Error),
    #[error("Error constructing query: {0}")]
    QueryError(#[from] surrealdb_abstraction::error::Error),
    #[error("OAuth error: {0}")]
    OAuthError(#[from] crate::auth::oauth::error::OauthError),
    #[error("Serialization error: {0}")]
    FormSerializationError(#[from] serde_urlencoded::ser::Error),
    #[error("Internal error: {0}")]
    InternalError(String),
    #[error("Not found")]
    NotFound,
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Not implemented")]
    NotImplemented,
    #[error("Not implemented: {0}")]
    NotImplementedWithMessage(String),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Unauthorized: {0}")]
    UnauthorizedWithMessage(String),
    #[error("Content type not accepted")]
    NotAcceptable,
    #[error(transparent)]
    GetIdentityError(#[from] actix_identity::error::GetIdentityError),
    #[error(transparent)]
    GenericError(#[from] anyhow::Error),
}

impl ResponseError for ServerResponseError {
    fn status_code(&self) -> StatusCode {
        match self {
            ServerResponseError::NotFound => StatusCode::NOT_FOUND,
            ServerResponseError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ServerResponseError::Unauthorized => StatusCode::UNAUTHORIZED,
            ServerResponseError::UnauthorizedWithMessage(_) => StatusCode::UNAUTHORIZED,
            ServerResponseError::NotImplemented => StatusCode::NOT_IMPLEMENTED,
            ServerResponseError::NotImplementedWithMessage(_) => StatusCode::NOT_IMPLEMENTED,
            ServerResponseError::NotAcceptable => StatusCode::NOT_ACCEPTABLE,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}
