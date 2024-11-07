pub(crate) mod engine;

use crate::error::ServerResponseError;
use crate::extractors::Authenticated;
use actix_web::{HttpResponse, Responder};
use async_graphql_actix_web::GraphQLRequest;
use helper_macros::generate_endpoint;
use reqwest::Client;
use serde_json::Value;
use tracing::error;

async fn send_graphql_request(
    client: &Client,
    endpoint: String,
    request: GraphQLRequest,
) -> Result<Value, ServerResponseError> {
    let data = request.into_inner();

    let response = client
        .post(endpoint)
        .header("Content-Type", "application/json")
        .header("surreal-ns", "default")
        .header("surreal-db", "default")
        .basic_auth("root", Some("root"))
        .json(&data)
        .send()
        .await?;

    if !response.status().is_success() {
        let body = response.text().await.unwrap_or_default();
        error!("Error forwarding GraphQL request: {}", body);
        return Err(ServerResponseError::InternalError(format!(
            "Error forwarding GraphQL request: {}",
            body
        )));
    }

    let json: Value = match response.json().await {
        Ok(json) => json,
        Err(e) => {
            error!("Error parsing GraphQL response: {}", e);
            return Err(e.into());
        }
    };
    Ok(json)
}

async fn handle_req(data: GraphQLRequest) -> impl Responder {
    let endpoint = format!(
        "http://{}/graphql",
        tosic_utils::prelude::env!("SURREALDB_URL")
    );
    let client = Client::new();

    match send_graphql_request(&client, endpoint, data).await {
        Ok(response_json) => HttpResponse::Ok().json(response_json),
        Err(e) => {
            error!("Error forwarding GraphQL request: {:?}", e);
            HttpResponse::InternalServerError().body("Internal Server Error")
        }
    }
}

generate_endpoint! {
    fn graphql_endpoint;
    method: post;
    path: "/graphql";
    docs: {
        responses: {
            (status = 200, description = "Successful GraphQL request"),
            (status = 401, description = "Invalid credentials"),
            (status = 500, description = "Something happened when forwarding the GraphQL request"),
        },
        security: [
            ("bearer_token" = []),
            ("cookie_session" = []),
        ]
    }
    params: {
        _auth: Authenticated,
        request: GraphQLRequest
    }
    {
        Ok(handle_req(request).await)
    }
}
