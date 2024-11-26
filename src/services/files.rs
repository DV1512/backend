use crate::error::ServerResponseError;
use crate::models::file_metadata::FileMetadata;
use crate::services::user::get::get_user_by_token;
use std::sync::Arc;
use surrealdb::Surreal;

/// Inserts metadata for a file with filename `filename` and relates it
/// to the user holding token `token`.
pub async fn insert_file_metadata<T>(
    db: &Arc<Surreal<T>>,
    filename: String,
    token: String,
) -> Result<FileMetadata, ServerResponseError>
where
    T: surrealdb::Connection,
{
    let user = get_user_by_token(db, &token).await?;
    let user_id = user.id.ok_or(ServerResponseError::NotFound)?;

    const SQL: &str = "
        BEGIN TRANSACTION;
        LET $FILE = (CREATE file SET filename = $FILENAME);
        RELATE ($FILE) -> files_for -> ($USER);
        COMMIT TRANSACTION;
        SELECT * FROM $FILE;";

    let created: Option<FileMetadata> = db
        .query(SQL)
        .bind(("FILENAME", filename))
        .bind(("USER", user_id))
        .await?
        .take(2)?;
    created.ok_or(ServerResponseError::InternalError(
        "Error inserting file metadata into database".to_string(),
    ))
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
