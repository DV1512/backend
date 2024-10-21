#![allow(dead_code)]

use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};

#[derive(Debug, thiserror::Error)]
pub enum ServerResponseError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] surrealdb::Error),
    #[error("Error constructing query: {0}")]
    QueryError(#[from] surrealdb_abstraction::error::Error),
    #[error("OAuth error: {0}")]
    OAuthError(#[from] crate::auth::oauth::error::OauthError),
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
    #[error(transparent)]
    GenericError(#[from] anyhow::Error),
}

impl ResponseError for ServerResponseError {
    fn status_code(&self) -> StatusCode {
        match self {
            ServerResponseError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ServerResponseError::OAuthError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ServerResponseError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ServerResponseError::QueryError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ServerResponseError::NotFound => StatusCode::NOT_FOUND,
            ServerResponseError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ServerResponseError::GenericError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ServerResponseError::Unauthorized => StatusCode::UNAUTHORIZED,
            ServerResponseError::UnauthorizedWithMessage(_) => StatusCode::UNAUTHORIZED,
            ServerResponseError::NotImplemented => StatusCode::NOT_IMPLEMENTED,
            ServerResponseError::NotImplementedWithMessage(_) => StatusCode::NOT_IMPLEMENTED,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}
