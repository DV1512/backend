use actix_web::web::{self, JsonConfig};
use serde::{Deserialize, Serialize};

mod post;
use post::*;

pub fn embeddings_service() -> impl actix_web::dev::HttpServiceFactory {
    let json_config = JsonConfig::default().limit(usize::MAX);

    web::scope("/embeddings")
        .app_data(json_config)
        .service(add_embeddings)
        .service(search_embeddings)
}
