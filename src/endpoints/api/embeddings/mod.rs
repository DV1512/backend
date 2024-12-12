use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::{error::ServerResponseError, models::thing::Thing, state::AppState};

#[derive(Serialize, Deserialize)]
struct Entry {
    id: Option<Thing>,
    similarity: Option<f32>,
    embedding: Option<Vec<f32>>,

    mitre_id: String,
    mitre_name: String,
    mitre_url: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum EntryType {
    Threat,
    Mitigation,
}

impl From<EntryType> for String {
    fn from(t: EntryType) -> Self {
        let s = match t {
            EntryType::Threat => "threat",
            EntryType::Mitigation => "mitigation",
        };
        String::from(s)
    }
}

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
    let _: Vec<Entry> = state
        .db
        .insert(String::from(data.entry_type))
        .content(data.entries)
        .await?;
    Ok(HttpResponse::Created())
}

#[post("/search")]
async fn search_embeddings(
    web::Json(data): web::Json<SearchEmbeddingsRequest>,
    state: web::Data<AppState>,
) -> Result<impl Responder, ServerResponseError> {
    let sql = format!(
        "
        SELECT mitre_id, mitre_name, mitre_url,
        vector::similarity::cosine(embedding, $query_embedding) AS similarity
        FROM {} 
        WHERE embedding <|{},40|> $query_embedding;",
        String::from(data.entry_type),
        data.num_neighbors
    );
    let entries: Vec<Entry> = state
        .db
        .query(sql)
        .bind(("query_embedding", data.embedding))
        .await?
        .take(0)?;

    Ok(HttpResponse::Ok().json(entries))
}

pub fn embeddings_service() -> impl actix_web::dev::HttpServiceFactory {
    web::scope("/embeddings")
        .service(add_embeddings)
        .service(search_embeddings)
}
