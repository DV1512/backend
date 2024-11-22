use crate::auth::users::get::utils::get_user_by_token;
use crate::error::ServerResponseError;
use crate::extractors::AuthenticatedToken;
use crate::extractors::IntoSession;
use crate::models::datetime::Datetime;
use crate::models::thing::Thing;
use crate::state::AppState;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::MultipartForm;
use actix_web::dev::HttpServiceFactory;
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use surrealdb::Surreal;

fn get_file_upload_path(filename: &dyn ToString) -> PathBuf {
    let upload_directory = tosic_utils::prelude::env!("FILES_UPLOAD_PATH", "/tmp/file_uploads");
    tracing::info!("Using upload directory: '{upload_directory}'");
    let upload_path = std::path::PathBuf::from(upload_directory);
    upload_path.join(filename.to_string())
}

/// Inserts metadata for a file with filename `filename` and relates it
/// to the user holding token `token`.
async fn insert<T>(
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
async fn get<T>(
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
async fn get_all_by_token<T>(
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
async fn delete<T>(
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

/// A type representing the metadata of an uploaded file.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct FileMetadata {
    id: Thing,
    /// The original filename of the uploaded file.
    /// For security reasons, uploaded files are
    /// persisted to disk with a filename equal
    /// to its database record ID.
    filename: String,
    /// The datetime when the file was uploaded.
    created_at: Datetime,
}

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(limit = "100MB")]
    file: TempFile,
}

#[post("")]
async fn upload_file(
    MultipartForm(form): MultipartForm<UploadForm>,
    auth: AuthenticatedToken,
    state: web::Data<AppState>,
) -> Result<impl Responder, ServerResponseError> {
    let filename: String = form.file.file_name.ok_or(ServerResponseError::BadRequest(
        "Uploaded file had no filename".to_string(),
    ))?;
    let token = auth.get_token();
    let metadata = insert(&state.db, filename, token).await?;
    let file_path = get_file_upload_path(&metadata.id.id);
    form.file
        .file
        .persist(file_path)
        .map_err(|e| ServerResponseError::InternalError(e.to_string()))?;
    Ok(HttpResponse::Created().json(metadata))
}

#[get("/{file_id}")]
async fn get_file(
    file_id: web::Path<String>,
    auth: AuthenticatedToken,
    state: web::Data<AppState>,
) -> Result<impl Responder, ServerResponseError> {
    let file_id = file_id.into_inner();
    let token = auth.get_token();
    let file_metadata = get(&state.db, file_id, token).await?;
    Ok(HttpResponse::Ok().json(file_metadata))
}

#[delete("/{file_id}")]
async fn delete_file(
    file_id: web::Path<String>,
    auth: AuthenticatedToken,
    state: web::Data<AppState>,
) -> Result<impl Responder, ServerResponseError> {
    let file_id = file_id.into_inner();
    let token = auth.get_token();
    delete(&state.db, file_id.clone(), token).await?;
    let file_path = get_file_upload_path(&file_id);
    fs::remove_file(&file_path).map_err(|e| ServerResponseError::InternalError(e.to_string()))?;
    Ok(HttpResponse::Ok().finish())
}

#[get("")]
async fn list_files(
    auth: AuthenticatedToken,
    state: web::Data<AppState>,
) -> Result<impl Responder, ServerResponseError> {
    let token = auth.get_token();
    let files = get_all_by_token(&state.db, token).await?;
    Ok(HttpResponse::Ok().json(files))
}

#[get("/{file_id}/content")]
async fn download_file(
    file_id: web::Path<String>,
    auth: AuthenticatedToken,
    state: web::Data<AppState>,
) -> Result<impl Responder, ServerResponseError> {
    let token = auth.get_token();
    let metadata = get(&state.db, file_id.into_inner(), token).await?;
    let file_path = get_file_upload_path(&metadata.id.id);
    let file = actix_files::NamedFile::open_async(file_path)
        .await
        .map_err(|_| ServerResponseError::NotFound)?;
    Ok(file)
}

/// A simple file storage service.
/// Operations:
/// * Upload file
/// * Delete file and metadata
/// * Get metadata
/// * Get all metadata for user
/// * Download file
pub fn files_service() -> impl HttpServiceFactory {
    web::scope("/files")
        .service(upload_file)
        .service(get_file)
        .service(delete_file)
        .service(list_files)
        .service(download_file)
}
