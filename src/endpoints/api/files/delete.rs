use std::fs;

use actix_web::{delete, get, web, HttpResponse, Responder};

use crate::{
    endpoints::files, error::ServerResponseError, models::UserSession,
    services::files::delete::delete_file_metadata, state::AppState,
};

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
pub async fn delete_file(
    file_id: web::Path<String>,
    session: UserSession,
    state: web::Data<AppState>,
) -> Result<impl Responder, ServerResponseError> {
    let file_id = file_id.into_inner();
    delete_file_metadata(&state.db, file_id.clone(), session.user_id).await?;
    let file_path = state.files.get_path_for(&file_id);
    if let Err(err) = fs::remove_file(&file_path) {
        return Err(ServerResponseError::InternalError(err.to_string()));
    }
    Ok(HttpResponse::Ok().finish())
}
