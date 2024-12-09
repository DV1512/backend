use crate::error::ServerResponseError;
use crate::extractors::AuthenticatedToken;
use crate::extractors::IntoSession;
use crate::generate_endpoint;
use crate::models::UserInfo;
use crate::services::user::delete::delete_user;
use crate::services::user::get::get_user_by_token;
use crate::state::AppState;
use actix_web::{web, HttpResponse};
use utoipa::ToSchema;

generate_endpoint! {
    fn delete_user_endpoint;
    method: delete;
    path: "";
    docs: {
        tag: "user",
        responses: {
            (status = 200, description = "User deleted successfully"),
            (status = 401, description = "Not logged in"),
            (status = 404, description = "User not found or invalid credentials"),
            (status = 500, description = "An error occurred when deleting user in the database"),
        },
        security: [
            ("bearer_token" = []),
            ("cookie_session" = []),
        ]
    }
    params: {
        token: AuthenticatedToken,
        state: web::Data<AppState>,
    };
    {
        let access_token = token.get_token();
        let user = get_user_by_token(&state.db, &access_token).await?;
        let Some(user_id) = user.id else {
            return Err(ServerResponseError::NotFound);
        };
        delete_user(&state.db , user_id).await?;
        Ok(HttpResponse::Ok().finish())
    }
}
