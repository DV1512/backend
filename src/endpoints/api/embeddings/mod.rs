use actix_web::web;
use serde::{Deserialize, Serialize};
use utoipa::{openapi, OpenApi};

use crate::models::{Entry, EntryType, MITREEntry};

mod post;
mod search;

use post::*;
use search::*;

pub fn embeddings_service() -> impl actix_web::dev::HttpServiceFactory {
    web::scope("/embeddings")
        .service(add_embeddings)
        .service(search_embeddings)
}

#[derive(OpenApi)]
#[openapi(
    paths(add_embeddings, search_embeddings),
    components(schemas(Entry, EntryType, MITREEntry),)
)]
pub(crate) struct EmbeddingsApi;
