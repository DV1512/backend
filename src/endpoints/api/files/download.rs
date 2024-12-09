use std::fs;

use actix_web::{delete, get, web, HttpResponse, Responder};
use helper_macros::generate_endpoint;

use crate::{
    endpoints::files, error::ServerResponseError, models::UserSession,
    services::files::get::get_file_metadata, state::AppState,
};

generate_endpoint! {
    fn download_file;
    method: get;
    path: "/{file_id}/content";
    docs: {
        params: (),
        tag: "files",
        responses: {
            (status = 200, description = "File found successfully"),
            (status = 401, description = "Unauthorized"),
            (status = 404, description = "File not found"),
            (status = 500, description = "Internal server error"),
        },
        security: [
            ("bearer_token" = []),
            ("cookie_session" = []),
        ]
    }
    params: {
        file_id: web::Path<String>,
        session: UserSession,
        state: web::Data<AppState>,
    };
    {
        let metadata = get_file_metadata(&state.db, file_id.into_inner(), session.user_id).await?;
        let file_path = state.files.get_path_for(&metadata.id.id.to_string());
        let Ok(file) = actix_files::NamedFile::open_async(file_path).await else {
            return Err(ServerResponseError::NotFound);
        };
        Ok(file)
    }
}
