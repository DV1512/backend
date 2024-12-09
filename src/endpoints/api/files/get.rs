use actix_web::{get, web, HttpResponse, Responder};

use crate::{
    endpoints::files,
    error::ServerResponseError,
    models::{FileMetadata, FileMetadataMultiple, UserSession},
    services::files::get::{get_file_metadata, get_file_metadata_by_token},
    state::AppState,
};

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
pub async fn get_file(
    file_id: web::Path<String>,
    session: UserSession,
    state: web::Data<AppState>,
) -> Result<impl Responder, ServerResponseError> {
    let file_id = file_id.into_inner();
    let file_metadata = get_file_metadata(&state.db, file_id, session.user_id).await?;
    Ok(HttpResponse::Ok().json(file_metadata))
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
pub async fn list_files(
    session: UserSession,
    state: web::Data<AppState>,
) -> Result<impl Responder, ServerResponseError> {
    let files = get_file_metadata_by_token(&state.db, session.user_id).await?;
    Ok(HttpResponse::Ok().json(files))
}
