use std::fs;

use actix_web::{delete, get, web, HttpResponse, Responder};
use helper_macros::generate_endpoint;

use crate::{
    endpoints::files, error::ServerResponseError, models::UserSession,
    services::files::delete::delete_file_metadata, state::AppState,
};

generate_endpoint! {
    fn delete_file;
    method: delete;
    path: "/{file_id}";
    docs: {
        params: (),
        tag: "files",
        responses: {
            (status = 200, description = "File deleted successfully"),
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
        delete_file_metadata(&state.db, file_id.clone(), session.user_id).await?;
        let file_path = state.files.get_path_for(&file_id);
        if let Err(err) = fs::remove_file(&file_path) {
            return Err(ServerResponseError::InternalError(err.to_string()));
        }
        Ok(HttpResponse::Ok().finish())
    }
}
