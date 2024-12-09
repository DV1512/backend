use actix_multipart::form::MultipartForm;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use helper_macros::generate_endpoint;

use crate::{
    dto::file_upload_form::UploadForm,
    endpoints::files,
    error::ServerResponseError,
    extractors::token_from_request,
    models::{FileMetadataMultiple, UserSession},
    services::files::insert::insert_file_metadata,
    state::AppState,
};

generate_endpoint! {
    fn upload_files;
    method: post;
    path: "";
    docs: {
        params: (),
        tag: "files",
        responses: {
            (status = 201, response = FileMetadataMultiple),
            (status = 401, description = "Unauthorized"),
            (status = 500, description = "Internal server error"),
        },
        security: [
            ("bearer_token" = []),
            ("cookie_session" = []),
        ]
    };
    params: {
        form: MultipartForm<UploadForm>,
        req: HttpRequest,
        state: web::Data<AppState>,
    };
    {
        let token = token_from_request(&state.db, &req).await?;
        let form = form.into_inner();

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
}
