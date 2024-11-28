use crate::error::ServerResponseError;
use crate::models::file_metadata::FileMetadata;
use crate::services::user::get::get_user_by_token;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use surrealdb::Surreal;

#[derive(Debug, Serialize, Deserialize)]
struct Filename {
    filename: String,
}

impl Filename {
    fn new(filename: &str) -> Self {
        Self {
            filename: filename.to_string(),
        }
    }
}

/// Inserts metadata for multiple files and relates them
/// to the user holding token `token`.
pub async fn insert_file_metadata<T>(
    db: &Arc<Surreal<T>>,
    filenames: Vec<&str>,
    token: String,
) -> Result<Vec<FileMetadata>, ServerResponseError>
where
    T: surrealdb::Connection,
{
    let user = get_user_by_token(db, &token).await?;
    let user_id = user.id.ok_or(ServerResponseError::NotFound)?;

    let files: Vec<Filename> = filenames
        .iter()
        .map(|filename| Filename::new(*filename))
        .collect();

    const SQL: &str = "
        BEGIN TRANSACTION;
        LET $FILE_RECORDS = (INSERT INTO file $FILES);
        RELATE ($FILE_RECORDS) -> files_for -> ($USER);
        COMMIT TRANSACTION;
        SELECT * FROM $FILE_RECORDS;
    ";

    let created: Vec<FileMetadata> = db
        .query(SQL)
        .bind(("FILES", files))
        .bind(("USER", user_id))
        .await?
        .take(2)?;

    Ok(created)
}

/// Returns the metadata of the file with ID `file_id` uploaded by
/// the user holding the token `token`.
pub async fn get_file_metadata<T>(
    db: &Arc<Surreal<T>>,
    file_id: String,
    token: String,
) -> Result<FileMetadata, ServerResponseError>
where
    T: surrealdb::Connection,
{
    let user = get_user_by_token(db, &token).await?;
    let user_id = user.id.ok_or(ServerResponseError::NotFound)?;
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
    token: String,
) -> Result<Vec<FileMetadata>, ServerResponseError>
where
    T: surrealdb::Connection,
{
    let user = get_user_by_token(db, &token).await?;
    let user_id = user.id.ok_or(ServerResponseError::NotFound)?;
    const SQL: &str = "SELECT VALUE in FROM files_for WHERE out = $USER FETCH in;";
    let files: Vec<FileMetadata> = db.query(SQL).bind(("USER", user_id)).await?.take(0)?;
    Ok(files)
}

/// Deletes the metadata of a file with ID `file_id` that
/// was uploaded by the user holding the token `token`.
pub async fn delete_file_metadata<T>(
    db: &Arc<Surreal<T>>,
    file_id: String,
    token: String,
) -> Result<(), ServerResponseError>
where
    T: surrealdb::Connection,
{
    let user = get_user_by_token(db, &token).await?;
    let user_id = user.id.ok_or(ServerResponseError::NotFound)?;
    const SQL: &str =
        "DELETE file WHERE meta::id(id) = $FILE AND ->files_for->user.id CONTAINS $USER;";
    db.query(SQL)
        .bind(("FILE", file_id))
        .bind(("USER", user_id))
        .await?;
    Ok(())
}
