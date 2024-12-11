use actix_web::{get, web, HttpResponse, Responder};
use helper_macros::generate_endpoint;

use crate::{
    endpoints::files,
    error::ServerResponseError,
    models::{FileMetadata, FileMetadataMultiple, UserSession},
    services::files::get::{get_file_metadata, get_file_metadata_by_token},
    state::AppState,
};

generate_endpoint! {
    fn get_file;
    method: get;
    path: "/{file_id}";
    docs: {
        params: (),
        tag: "files",
        responses: {
            (status = 200, response = FileMetadata),
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
        let file_id = file_id.into_inner();
        let file_metadata = get_file_metadata(&state.db, file_id, session.user_id).await?;
        Ok(HttpResponse::Ok().json(file_metadata))
    }
}

generate_endpoint! {
    fn list_files;
    method: get;
    path: "";
    docs: {
        params: (),
        tag: "files",
        responses: {
            (status = 200 , response = FileMetadataMultiple),
            (status = 401, description = "Unauthorized"),
            (status = 500, description = "Internal server error"),
        },
        security: [
            ("bearer_token" = []),
            ("cookie_session" = []),
        ]
    }
    params: {
        session: UserSession,
        state: web::Data<AppState>,
    };
    {
        let files = get_file_metadata_by_token(&state.db, session.user_id).await?;
        Ok(HttpResponse::Ok().json(files))
    }
}
