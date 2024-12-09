use actix_multipart::form::MultipartForm;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};

use crate::{
    dto::file_upload_form::UploadForm,
    endpoints::files,
    error::ServerResponseError,
    extractors::token_from_request,
    models::{FileMetadataMultiple, UserSession},
    services::files::insert::insert_file_metadata,
    state::AppState,
};

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
pub async fn upload_files(
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

    let metadata = insert_file_metadata(&state.db, filenames.clone(), token).await?;

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
