use crate::dto::file_upload_form::UploadForm;
use crate::error::ServerResponseError;
use crate::extractors::token_from_request;
use crate::extractors::AuthenticatedToken;
use crate::extractors::IntoSession;
use crate::models::datetime::Datetime;
use crate::models::file_metadata::FileMetadata;
use crate::models::FileMetadataMultiple;
use crate::models::UserSession;
use crate::services::files;
use crate::services::user::get::get_user_by_token;
use crate::state::AppState;
use actix_multipart::form::tempfile;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::MultipartForm;
use actix_web::dev::HttpServiceFactory;
use actix_web::HttpRequest;
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use utoipa::openapi;
use utoipa::path;
use utoipa::OpenApi;

pub struct FilesServiceState {
    pub upload_path: PathBuf,
}

impl FilesServiceState {
    pub fn new() -> Self {
        let env_upload_path = tosic_utils::prelude::env!("FILES_UPLOAD_PATH");
        let upload_path = std::path::PathBuf::from(env_upload_path);
        Self { upload_path }
    }

    pub fn get_path_for(&self, filename: &str) -> PathBuf {
        self.upload_path.join(filename)
    }
}

#[utoipa::path(
    get,
    path = "/(file_id)",
    tag = "files",
    responses (
        (status = 200, response = FileMetadata),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "File not found"),
        (status = 500, description = "Internal server error"),
    ),
)]
#[get("/{file_id}")]
async fn get_file(
    file_id: web::Path<String>,
    session: UserSession,
    state: web::Data<AppState>,
) -> Result<impl Responder, ServerResponseError> {
    let file_id = file_id.into_inner();
    let file_metadata = files::get::get_file_metadata(&state.db, file_id, session.user_id).await?;
    Ok(HttpResponse::Ok().json(file_metadata))
}

#[utoipa::path(
    delete,
    path = "/(file_id)",
    tag = "files",
    responses (
        (status = 200, description = "File deleted successfully"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "File not found"),
        (status = 500, description = "Internal server error"),
    ),
)]
#[delete("/{file_id}")]
async fn delete_file(
    file_id: web::Path<String>,
    session: UserSession,
    state: web::Data<AppState>,
) -> Result<impl Responder, ServerResponseError> {
    let file_id = file_id.into_inner();
    files::delete::delete_file_metadata(&state.db, file_id.clone(), session.user_id).await?;
    let file_path = state.files.get_path_for(&file_id);
    if let Err(err) = fs::remove_file(&file_path) {
        return Err(ServerResponseError::InternalError(err.to_string()));
    }
    Ok(HttpResponse::Ok().finish())
}

#[utoipa::path(
    get,
    path = "",
    tag = "files",
    responses (
        (status = 200 , response = FileMetadataMultiple),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error"),
    ),
)]
#[get("")]
async fn list_files(
    session: UserSession,
    state: web::Data<AppState>,
) -> Result<impl Responder, ServerResponseError> {
    let files = files::get::get_file_metadata_by_token(&state.db, session.user_id).await?;
    Ok(HttpResponse::Ok().json(files))
}

#[utoipa::path(
    get,
    path = "/(file_id)/content",
    tag = "files",
    responses (
        (status = 200),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "File not found"),
        (status = 500, description = "Internal server error"),
    ),
)]
#[get("/{file_id}/content")]
async fn download_file(
    file_id: web::Path<String>,
    session: UserSession,
    state: web::Data<AppState>,
) -> Result<impl Responder, ServerResponseError> {
    let metadata =
        files::get::get_file_metadata(&state.db, file_id.into_inner(), session.user_id).await?;
    let file_path = state.files.get_path_for(&metadata.id.id.to_string());
    let Ok(file) = actix_files::NamedFile::open_async(file_path).await else {
        return Err(ServerResponseError::NotFound);
    };
    Ok(file)
}

#[utoipa::path(
    post,
    path = "",
    tag = "files",
    responses (
        (status = 201, response = FileMetadataMultiple),
        (status = 401),
        (status = 500),
    ),
)]
#[post("")]
async fn upload_files(
    MultipartForm(form): MultipartForm<UploadForm>,
    req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<impl Responder, ServerResponseError> {
    let token = token_from_request(&state.db, &req).await?;

    let filenames: Vec<&str> = form
        .files
        .iter()
        .map(|temp_file| {
            temp_file
                .file_name
                .as_ref()
                .map_or("unnamed_file", |f| f.as_str())
        })
        .collect();

    let metadata = files::insert::insert_file_metadata(&state.db, filenames.clone(), token).await?;

    let persisted: Result<Vec<_>, _> = form
        .files
        .into_iter()
        .zip(&metadata)
        .map(|(f, m)| {
            let path = state.files.get_path_for(&m.id.id.to_string());
            f.file.persist(path)
        })
        .collect();

    if let Err(err) = persisted {
        return Err(ServerResponseError::InternalError(err.to_string()));
    }
    Ok(HttpResponse::Created().json(metadata))
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
        .service(upload_files)
        .service(get_file)
        .service(delete_file)
        .service(list_files)
        .service(download_file)
}

#[derive(OpenApi)]
#[openapi(
    paths(upload_files, get_file, delete_file, list_files, download_file),
    components(schemas(FileMetadata), responses(FileMetadata, FileMetadataMultiple))
)]
pub(crate) struct FilesApi;
