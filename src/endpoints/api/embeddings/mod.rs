use actix_web::web;
use serde::{Deserialize, Serialize};

mod post;
use post::*;

pub fn embeddings_service() -> impl actix_web::dev::HttpServiceFactory {
    web::scope("/embeddings")
        .service(add_embeddings)
        .service(search_embeddings)
}
