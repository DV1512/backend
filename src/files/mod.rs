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
use std::sync::Arc;
use surrealdb::Surreal;

// TODO: Replace with environment variable
const UPLOAD_DIRECTORY: &str = "/Users/gustav/uploads/";

async fn insert<T>(
    db: &Arc<Surreal<T>>,
    filename: String,
    user_id: Thing,
) -> Result<FileMetadata, ServerResponseError>
where
    T: surrealdb::Connection,
{
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

async fn get<T>(
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
/// corresponding to the supplied token.
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

/// Deletes a file metadata record with a supplied ID that
/// was uploaded by the user with corresponding token.
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

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FileMetadata {
    id: Thing,
    filename: String,
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
    let Some(filename) = form.file.file_name else {
        return Err(ServerResponseError::BadRequest(
            "Uploaded file had no filename".to_string(),
        ));
    };

    let token = auth.get_token();
    let user = get_user_by_token(&state.db, &token).await?;
    let user_id = user.id.ok_or(ServerResponseError::NotFound)?;

    let metadata = insert(&state.db, filename, user_id).await?;

    let upload_path = std::path::PathBuf::from(UPLOAD_DIRECTORY);
    let file_path = upload_path.join(metadata.id.id.to_string());

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
    let user = get_user_by_token(&state.db, &token).await?;
    let user_id = user.id.ok_or(ServerResponseError::NotFound)?;

    let file_metadata = get(&state.db, file_id, user_id).await?;
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

    let upload_path = std::path::PathBuf::from(UPLOAD_DIRECTORY);
    let file_path = upload_path.join(&file_id);
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

pub fn files_service() -> impl HttpServiceFactory {
    web::scope("/files")
        .service(upload_file)
        .service(get_file)
        .service(delete_file)
        .service(list_files)
}
