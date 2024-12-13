use crate::error::ServerResponseError;
use std::sync::Arc;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

/// Deletes the metadata of a file with ID `file_id` that
/// was uploaded by the user holding the token `token`.
pub async fn delete_file_metadata<T>(
    db: &Arc<Surreal<T>>,
    file_id: String,
    user_id: Thing,
) -> Result<(), ServerResponseError>
where
    T: surrealdb::Connection,
{
    const SQL: &str =
        "DELETE file WHERE meta::id(id) = $FILE AND ->files_for->user.id CONTAINS $USER;";
    db.query(SQL)
        .bind(("FILE", file_id))
        .bind(("USER", user_id))
        .await?;
    Ok(())
}
