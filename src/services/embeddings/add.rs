use crate::error::ServerResponseError;
use crate::models::{Entry, EntryType};
use std::sync::Arc;
use surrealdb::Surreal;

/// Inserts embeddings and their corresponding metadata into the database
pub async fn insert_embeddings<T>(
    db: &Arc<Surreal<T>>,
    entries: Vec<Entry>,
    entry_type: EntryType,
) -> Result<Vec<Entry>, ServerResponseError>
where
    T: surrealdb::Connection,
{
    let embeddings: Vec<Entry> = db.insert(entry_type.to_string()).content(entries).await?;
    Ok(embeddings)
}
