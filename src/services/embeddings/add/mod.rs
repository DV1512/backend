use super::EntryType;
use crate::error::ServerResponseError;
use crate::services::embeddings::Entry;
use std::sync::Arc;
use surrealdb::Surreal;

pub async fn insert_embeddings<T>(
    db: &Arc<Surreal<T>>,
    entries: Vec<Entry>,
    entry_type: EntryType,
) -> Result<Vec<Entry>, ServerResponseError>
where
    T: surrealdb::Connection,
{
    let embeddings: Vec<Entry> = db.insert(String::from(entry_type)).content(entries).await?;
    Ok(embeddings)
}
