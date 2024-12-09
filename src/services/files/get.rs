use crate::error::ServerResponseError;
use crate::models::file_metadata::FileMetadata;
use std::sync::Arc;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

/// Returns the metadata of the file with ID `file_id` uploaded by
/// the user holding the token `token`.
pub async fn get_file_metadata<T>(
    db: &Arc<Surreal<T>>,
    file_id: String,
    user_id: Thing,
) -> Result<FileMetadata, ServerResponseError>
where
    T: surrealdb::Connection,
{
    const SQL: &str =
        "SELECT VALUE in FROM files_for WHERE meta::id(in) = $FILE AND out = $USER FETCH in;";
    let found = db
        .query(SQL)
        .bind(("FILE", file_id))
        .bind(("USER", user_id))
        .await?
        .take(0)?;
    match found {
        Some(file) => Ok(file),
        None => Err(ServerResponseError::NotFound),
    }
}

/// Returns metadata of all files uploaded by the user
/// holding token `token`.
pub async fn get_file_metadata_by_token<T>(
    db: &Arc<Surreal<T>>,
    user_id: Thing,
) -> Result<Vec<FileMetadata>, ServerResponseError>
where
    T: surrealdb::Connection,
{
    const SQL: &str = "SELECT VALUE in FROM files_for WHERE out = $USER FETCH in;";
    let files: Vec<FileMetadata> = db.query(SQL).bind(("USER", user_id)).await?.take(0)?;
    Ok(files)
}
