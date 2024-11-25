use std::time::Duration;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder};
use actix_web::web::Bytes;
use awc::Client;
use awc::error::SendRequestError;
use awc::http::Method;
use tracing::debug;
use helper_macros::generate_endpoint;
use crate::error::ServerResponseError;

async fn proxy(path: &str, req: HttpRequest, bytes: Bytes) -> impl Responder {
    let client = Client::default();
    let url = format!("{}{}", req.url_for_static("llm").expect("LLM URL not set"), path);

    let req = client.request_from(url, req.head()).timeout(Duration::from_mins(20));

    let mut resp;

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

            let mut response;

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
    path: "/chat";
    /*docs: {
        tag: "chat";
        responses: {
            (status = 200, description = "Request successful")
        }
    }*/
    params: {
        req: HttpRequest,
        body: Bytes,
    };
    {
        debug!("Received chat request");
        Ok(proxy("chat/structured", req, body).await)
    }
}