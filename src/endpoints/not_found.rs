use actix_web::http::{header, StatusCode};
use actix_web::{HttpRequest, HttpResponse, Responder};
use tracing::error;

pub(crate) async fn not_found(req: HttpRequest) -> impl Responder {
    let path = req.path();
    error!("Path not found: {}", path);

    let html = format!(
        r#"
        <!doctype html>
        <html>
            <head>
                <title>Error 404</title>
            </head>
            <body>
                <h1>404 - Endpoint not defined</h1>
                <p>Path: {path}</p>
            </body>
        </html>
        "#,
    );

    HttpResponse::build(StatusCode::NOT_FOUND)
        .insert_header(header::ContentType::html())
        .body(html)
}
