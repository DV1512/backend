pub(crate) mod engine;

use serde::{Serialize, Deserialize};
use crate::error::ServerResponseError;
use crate::extractors::Authenticated;
use actix_web::{HttpResponse, Responder};
use actix_web_httpauth::extractors::basic::BasicAuth;
use actix_web_httpauth::headers::authorization::Basic;
use api_forge::{ApiRequest, Request};
use async_graphql_actix_web::GraphQLRequest;
use helper_macros::generate_endpoint;
use reqwest::Client;
use reqwest::header::ACCEPT;
use serde_json::{json, Value};
use tracing::error;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "lowercase")]
enum NameSpace {
    #[default]
    Default
}
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "lowercase")]
enum Database {
    #[default]
    Default
}
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "lowercase")]
enum AccessMethod {
    #[default]
    User
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, Request)]
#[request(endpoint = "/signin", method = POST, response_type = TokenRes, transmission = Json)]
struct DBAuthReq {
    ns: NameSpace,
    db: Database,
    ac: AccessMethod,
    email: String,
    password: String
}

impl DBAuthReq {
    pub fn new(email: String, password: String) -> Self {
        DBAuthReq {
            email,
            password,
            ..Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TokenRes {
	pub code: i32,

	pub details: String,

	pub token: Option<String>,
    pub information: Option<String>,
}


async fn send_graphql_request(
    client: &Client,
    endpoint: String,
    request: GraphQLRequest,
    token: String,
) -> Result<Value, ServerResponseError> {
    let data = request.into_inner();

    let response = client
        .post(endpoint)
        .header("Content-Type", "application/json")
        .header("surreal-ns", "default")
        .header("surreal-db", "default")
        .bearer_auth(token)
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

async fn handle_req(data: GraphQLRequest, auth: BasicAuth) -> impl Responder {
    let endpoint = format!(
        "http://{}/graphql",
        tosic_utils::prelude::env!("SURREALDB_URL")
    );
    let basic_auth = Basic::new("test@test.com", Some("test"));
    let client = Client::new();

    let user = auth.user_id();
    let pass = auth.password();

    let db_auth_req = DBAuthReq::new(user.into(), pass.unwrap_or_default().to_string());

    let res = db_auth_req.send_and_parse("http://localhost:7352", None, None).await;

    let token = match res {
        Ok(token) => token,
        Err(e) => {
            error!("Error Authenticating: {:?}", e);
            return HttpResponse::Unauthorized().body("Invalid credentials");
        }
    };

    if token.token.is_none() && token.information.is_some() {
        error!("Error Authenticating: {:?}", token);
        return HttpResponse::Unauthorized().body(token.information.unwrap());
    } else if token.token.is_none() && token.information.is_none() {
        error!("Error Authenticating: {:?}", token);
        return HttpResponse::InternalServerError().body("Internal Server Error");
    }

    match send_graphql_request(&client, endpoint, data, token.token.unwrap()).await {
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
        basic_auth: BasicAuth,
        request: GraphQLRequest
    }
    {
        //let basic_auth = Basic::new("test@test.com", Some("test"));
        Ok(handle_req(request, basic_auth.into()).await)
    }
}
