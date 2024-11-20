use crate::error::ServerResponseError;
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

#[derive(Serialize)]
struct Content {
    filename: String,
}
async fn insert<T>(
    db: &Arc<Surreal<T>>,
    filename: String,
) -> Result<FileMetadata, ServerResponseError>
where
    T: surrealdb::Connection,
{
    let created = db.create("file").content(Content { filename }).await?;
    match created {
        Some(id) => Ok(id),
        None => Err(ServerResponseError::InternalError(
            "Error creating file metadata".to_string(),
        )),
    }
}

async fn get<T>(db: &Arc<Surreal<T>>, id: String) -> Result<FileMetadata, ServerResponseError>
where
    T: surrealdb::Connection,
{
    let found: Option<FileMetadata> = db.select(("file", id)).await?;
    match found {
        Some(file) => Ok(file),
        None => Err(ServerResponseError::NotFound),
    }
}

async fn delete<T>(db: &Arc<Surreal<T>>, id: String) -> Result<(), ServerResponseError>
where
    T: surrealdb::Connection,
{
    let deleted: Option<FileMetadata> = db.delete(("file", id)).await?;
    match deleted {
        Some(_) => Ok(()),
        None => Err(ServerResponseError::NotFound),
    }
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
    state: web::Data<AppState>,
    MultipartForm(form): MultipartForm<UploadForm>,
) -> Result<impl Responder, ServerResponseError> {
    let Some(filename) = form.file.file_name else {
        return Err(ServerResponseError::BadRequest(
            "Uploaded file had no filename".to_string(),
        ));
    };
    let metadata = insert(&state.db, filename).await?;

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
    state: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<impl Responder, ServerResponseError> {
    let id = id.into_inner();
    let file_metadata = get(&state.db, id).await?;
    Ok(HttpResponse::Ok().json(file_metadata))
}

#[delete("/{file_id}")]
async fn delete_file(
    state: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<impl Responder, ServerResponseError> {
    let id = id.into_inner();
    let upload_path = std::path::PathBuf::from(UPLOAD_DIRECTORY);
    let file_path = upload_path.join(&id);
    fs::remove_file(&file_path).map_err(|e| ServerResponseError::InternalError(e.to_string()))?;

    delete(&state.db, id.clone()).await?;

    Ok(HttpResponse::Ok().finish())
}

#[get("")]
async fn list_files(state: web::Data<AppState>) -> Result<impl Responder, ServerResponseError> {
    let res: Vec<FileMetadata> = state.db.select("file").await?;
    Ok(HttpResponse::Ok().json(res))
}

pub fn files_service() -> impl HttpServiceFactory {
    web::scope("/files")
        .service(upload_file)
        .service(get_file)
        .service(delete_file)
        .service(list_files)
}
