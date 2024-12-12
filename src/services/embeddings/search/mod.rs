use crate::error::ServerResponseError;
use crate::services::embeddings::Entry;
use crate::services::embeddings::EntryType;
use std::sync::Arc;
use surrealdb::Surreal;

pub async fn search_embeddings_<T>(
    db: &Arc<Surreal<T>>,
    embedding: Vec<f32>,
    entry_type: EntryType,
    num_neighbors: u32,
) -> Result<Vec<Entry>, ServerResponseError>
where
    T: surrealdb::Connection,
{
    let sql = format!(
        "
        SELECT mitre_id, mitre_name, mitre_description, mitre_url,
        vector::similarity::cosine(embedding, $query_embedding) AS similarity
        FROM {} 
        WHERE embedding <|{},40|> $query_embedding;",
        String::from(entry_type),
        num_neighbors
    );
    let entries: Vec<Entry> = db
        .query(sql)
        .bind(("query_embedding", embedding))
        .await?
        .take(0)?;

    Ok(entries)
}
