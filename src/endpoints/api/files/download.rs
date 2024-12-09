use std::fs;

use actix_web::{delete, get, web, HttpResponse, Responder};

use crate::{
    endpoints::files, error::ServerResponseError, models::UserSession,
    services::files::get::get_file_metadata, state::AppState,
};
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
pub async fn download_file(
    file_id: web::Path<String>,
    session: UserSession,
    state: web::Data<AppState>,
) -> Result<impl Responder, ServerResponseError> {
    let metadata = get_file_metadata(&state.db, file_id.into_inner(), session.user_id).await?;
    let file_path = state.files.get_path_for(&metadata.id.id.to_string());
    let Ok(file) = actix_files::NamedFile::open_async(file_path).await else {
        return Err(ServerResponseError::NotFound);
    };
    Ok(file)
}
