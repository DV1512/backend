use crate::error::ServerResponseError;
use actix_web::{HttpResponse, Responder};
use api_forge::{ApiRequest, Request};
use serde::Serialize;
use tracing::error;

pub(crate) async fn check_health() -> Result<impl Responder, ServerResponseError> {
    #[derive(Request, Serialize, Debug)]
    #[request(endpoint = "/health")]
    struct DbHealthCheck;

    let request = DbHealthCheck;

    let mut url = tosic_utils::prelude::env!("SURREALDB_URL");

    if !url.starts_with("http://") || !url.starts_with("https://") {
        url = format!("http://{}", url);
    }

    request
        .send_request(url.as_str(), None, None)
        .await
        .map_err(|err| {
            error!("Database not responding, error: {}", err);
            ServerResponseError::FailedDependencyWithMessage("Database not responding".into())
        })?;

    Ok(HttpResponse::Ok())
}
