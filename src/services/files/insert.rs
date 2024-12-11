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
        .map(|filename| Filename {
            filename: filename.to_string(),
        })
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
