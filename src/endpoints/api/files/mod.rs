use crate::error::ServerResponseError;
use crate::extractors::AuthenticatedToken;
use crate::extractors::IntoSession;
use crate::models::datetime::Datetime;
use crate::models::file_metadata::FileMetadata;
use crate::models::thing::Thing;
use crate::services::files::*;
use crate::services::user::get::get_user_by_token;
use crate::state::AppState;
use actix_multipart::form::json::Json as MpJson;
use actix_multipart::form::tempfile::TempFile;
use actix_multipart::form::text::Text as MpText;
use actix_multipart::form::MultipartForm;
use actix_web::dev::HttpServiceFactory;
use actix_web::web::Json;
use actix_web::FromRequest;
use actix_web::HttpRequest;
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use helper_macros::generate_endpoint;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use surrealdb::Surreal;

#[derive(Debug, MultipartForm)]
struct UploadForm {
    #[multipart(limit = "100MB")]
    file: TempFile,
}

fn get_file_upload_path(filename: &dyn ToString) -> PathBuf {
    let upload_directory = tosic_utils::prelude::env!("FILES_UPLOAD_PATH", "/tmp/file_uploads");
    tracing::info!("Using upload directory: '{upload_directory}'");
    let upload_path = std::path::PathBuf::from(upload_directory);
    upload_path.join(filename.to_string())
}

#[post("")]
async fn upload_file(
    MultipartForm(form): MultipartForm<UploadForm>,
    req: HttpRequest,
    state: web::Data<AppState>,
) -> Result<impl Responder, ServerResponseError> {
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
    let _ = get_user_by_token(&state.db, bearer_token).await?;

    let filename: String = form.file.file_name.ok_or(ServerResponseError::BadRequest(
        "Uploaded file had no filename".to_string(),
    ))?;
    let metadata = insert(&state.db, filename, bearer_token.to_string()).await?;
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
