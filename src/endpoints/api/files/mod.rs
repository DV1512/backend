use crate::error::ServerResponseError;
use crate::extractors::AuthenticatedToken;
use crate::extractors::IntoSession;
use crate::models::datetime::Datetime;
use crate::models::file_metadata::FileMetadata;
use crate::services::files::*;
use crate::services::user::get::get_user_by_token;
use crate::state::AppState;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::MultipartForm;
use actix_web::dev::HttpServiceFactory;
use actix_web::HttpRequest;
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use surrealdb::Surreal;

pub struct FilesServiceState {
    pub upload_path: PathBuf,
}

impl FilesServiceState {
    pub fn new() -> Self {
        let env_upload_path = tosic_utils::prelude::env!("FILES_UPLOAD_PATH", "/tmp/file_uploads");
        let upload_path = std::path::PathBuf::from(env_upload_path);
        Self { upload_path }
    }

    pub fn get_path_for(&self, filename: &str) -> PathBuf {
        self.upload_path.join(filename)
    }
}

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(limit = "100MB")]
    file: TempFile,
}

async fn token_from_request<T>(
    db: &Arc<Surreal<T>>,
    req: &HttpRequest,
) -> Result<String, ServerResponseError>
where
    T: surrealdb::Connection,
{
    // The reason that we cannot enforce authentication for this endpoint
    // using an 'AuthenticatedToken' is because of the multipart file upload.
    // The process of uploading a file via multipart HTTP consists
    // of (at least) two requests.
    // For the first request, the client sends a "Expect: expect-100" header.
    // This header communicates to the server that it should
    // return "HTTP 100 Continue" if the user has sent an 'Authorization'
    // header (if authorization is required) and that the file to be uploaded
    // does not exceed file size limits.
    // If the first success is successfull, the client will send the contents
    // of the file to be uploaded with the following requests.
    // I believe that the problem is that the 'Authorization' header is sent
    // only for the first request, and not for any subsequent requests.
    // This results in the client first recieving a "HTTP 100 Continue",
    // followed by a "HTTP 401 Unauthorized".

    let Some(auth_header) = req.headers().get("authorization") else {
        return Err(ServerResponseError::Unauthorized);
    };
    let Ok(auth_value) = auth_header.to_str() else {
        return Err(ServerResponseError::BadRequest(
            "Invalid authorization header value".to_string(),
        ));
    };
    let Some(bearer_token) = auth_value.strip_prefix("Bearer ") else {
        return Err(ServerResponseError::BadRequest(
            "No bearer token in authorization header".to_string(),
        ));
    };
    if get_user_by_token(db, bearer_token).await.is_err() {
        return Err(ServerResponseError::Unauthorized);
    }
    Ok(bearer_token.to_string())
}

#[post("")]
async fn upload_file(
    MultipartForm(form): MultipartForm<UploadForm>,
    req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<impl Responder, ServerResponseError> {
    let filename: String = form.file.file_name.ok_or(ServerResponseError::BadRequest(
        "Uploaded file had no filename".to_string(),
    ))?;
    let token = token_from_request(&state.db, &req).await?;
    let metadata = insert_file_metadata(&state.db, filename, token).await?;
    let file_path = state.files.get_path_for(&metadata.id.id.to_string());
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
    let file_metadata = get_file_metadata(&state.db, file_id, token).await?;
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
    delete_file_metadata(&state.db, file_id.clone(), token).await?;
    let file_path = state.files.get_path_for(&file_id);
    fs::remove_file(&file_path).map_err(|e| ServerResponseError::InternalError(e.to_string()))?;
    Ok(HttpResponse::Ok().finish())
}

#[get("")]
async fn list_files(
    auth: AuthenticatedToken,
    state: web::Data<AppState>,
) -> Result<impl Responder, ServerResponseError> {
    let token = auth.get_token();
    let files = get_file_metadata_by_token(&state.db, token).await?;
    Ok(HttpResponse::Ok().json(files))
}

#[get("/{file_id}/content")]
async fn download_file(
    file_id: web::Path<String>,
    auth: AuthenticatedToken,
    state: web::Data<AppState>,
) -> Result<impl Responder, ServerResponseError> {
    let token = auth.get_token();
    let metadata = get_file_metadata(&state.db, file_id.into_inner(), token).await?;
    let file_path = state.files.get_path_for(&metadata.id.id.to_string());
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
