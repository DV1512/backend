use std::time::Duration;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse, Responder};
use actix_web::web::Bytes;
use awc::Client;
use awc::error::SendRequestError;
use awc::http::Method;
use tracing::debug;
use helper_macros::generate_endpoint;
use crate::dto::chat_request::ChatRequest;
use crate::error::ServerResponseError;
use crate::extractors::Authenticated;

async fn proxy(path: &str, req: HttpRequest, bytes: Bytes) -> impl Responder {
    let client = Client::default();
    let url = format!("{}{}", req.url_for_static("llm").expect("LLM URL not set"), path);

    let req = client.request_from(url, req.head()).timeout(Duration::from_mins(20));

    let resp;

    if !bytes.is_empty() {
        resp = req.send_body(bytes);
    } else {
        resp = req.send();
    }

    match resp.await {
        Ok(res) => {
            let content_type = res.content_type().to_string();
            debug!("Got response, content type: {}", content_type);

            let mut client_res = HttpResponse::build(res.status());

            let response;

            response = client_res.content_type(content_type);

            for (key, value) in res.headers().iter() {
                response.append_header((key.clone(), value.clone()));
            }

            response.streaming(res)
        },
        Err(err) => match err {
            SendRequestError::Http(http_err) => {
                HttpResponse::BadGateway().body(format!("HTTP error: {}", http_err))
            }
            _ => HttpResponse::InternalServerError().body(format!("Request failed: {}", err)),
        },
    }
}

generate_endpoint! {
    fn chat;
    method: post;
    path: "/chat/completions";
    docs: {
        tag: "llm",
        context_path: "/",
        responses: {
            (status = 200, description = "Everything works just fine!")
        }
    }
    params: {
        req: HttpRequest,
        body: web::Json<ChatRequest>,
        _auth: Authenticated,
    };
    {
        let body = serde_json::to_vec(&body.into_inner()).expect("Failed to serialize ChatRequest");

        debug!("Received chat request");
        Ok(proxy("chat/completions", req, Bytes::from(body)).await)
    }
}