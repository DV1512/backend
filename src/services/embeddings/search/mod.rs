use crate::error::ServerResponseError;
use crate::services::embeddings::{EntryType, MITREEntry};
use std::sync::Arc;
use surrealdb::Surreal;

/// Given an embedding, searches for relevant context from
/// the database
pub async fn search_embeddings_<T>(
    db: &Arc<Surreal<T>>,
    embedding: Vec<f32>,
    entry_type: EntryType,
    num_neighbors: u32,
) -> Result<Vec<MITREEntry>, ServerResponseError>
where
    T: surrealdb::Connection,
{
    let sql = format!(
        "
        SELECT mitre_id, mitre_name, mitre_description, mitre_url
        FROM {} 
        WHERE embedding <|{},40|> $query_embedding;",
        String::from(entry_type),
        num_neighbors
    );
    let entries: Vec<MITREEntry> = db
        .query(sql)
        .bind(("query_embedding", embedding))
        .await?
        .take(0)?;

    Ok(entries)
}
