use actix_web::{post, web, HttpResponse, Responder};

use crate::{
    dto::embeddings::{AddEmbeddingsRequest, SearchEmbeddingsRequest},
    error::ServerResponseError,
    services::embeddings::{add::insert_embeddings, search::search_embeddings_},
    state::AppState,
};

#[post("")]
pub async fn add_embeddings(
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
