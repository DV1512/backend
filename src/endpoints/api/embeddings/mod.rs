use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::{
    error::ServerResponseError,
    models::thing::Thing,
    services::embeddings::{add::insert_embeddings, search::search_embeddings_, Entry, EntryType},
    state::AppState,
};

#[derive(Serialize, Deserialize)]
struct AddEmbeddingsRequest {
    #[serde(rename = "type")]
    entry_type: EntryType,

    entries: Vec<Entry>,
}

#[derive(Serialize, Deserialize)]
struct SearchEmbeddingsRequest {
    #[serde(rename = "type")]
    entry_type: EntryType,

    embedding: Vec<f32>,
    num_neighbors: u32,
}

#[post("")]
async fn add_embeddings(
    web::Json(data): web::Json<AddEmbeddingsRequest>,
    state: web::Data<AppState>,
) -> Result<impl Responder, ServerResponseError> {
    insert_embeddings(&state.db, data.entries, data.entry_type).await?;
    Ok(HttpResponse::Created())
}

#[post("/search")]
async fn search_embeddings(
    web::Json(data): web::Json<SearchEmbeddingsRequest>,
    state: web::Data<AppState>,
) -> Result<impl Responder, ServerResponseError> {
    let embeddings = search_embeddings_(
        &state.db,
        data.embedding,
        data.entry_type,
        data.num_neighbors,
    )
    .await?;
    Ok(HttpResponse::Ok().json(embeddings))
}

pub fn embeddings_service() -> impl actix_web::dev::HttpServiceFactory {
    web::scope("/embeddings")
        .service(add_embeddings)
        .service(search_embeddings)
}
