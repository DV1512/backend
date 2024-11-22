use crate::auth::users::get::utils::get_user_by_token;
use crate::error::ServerResponseError;
use crate::extractors::AuthenticatedToken;
use crate::extractors::IntoSession;
use crate::generate_endpoint;
use crate::state::AppState;
use actix_web::{web, HttpResponse};
use utoipa::ToSchema;
use crate::dto::user_update_request::UserUpdateRequest;
use crate::services::update_user::update_user_data;

generate_endpoint! {
    fn update_user;
    method: put;
    path: "";
    docs: {
        tag: "oauth",
        responses: {
            (status = 200, description = "User updated successfully"),
            (status = 401, description = "Not logged in"),
            (status = 404, description = "User not found or invalid credentials"),
            (status = 500, description = "An error occurred when updating user information in the database"),
        },
        security: [
            ("bearer_token" = []),
            ("cookie_session" = []),
        ]
    }
    params: {
        data: web::Json<UserUpdateRequest>,
        token: AuthenticatedToken,
        state: web::Data<AppState>,
    };
    {
        let access_token = token.get_token();
        let user = get_user_by_token(&state.db, &access_token).await?;
        let Some(user_id) = user.id else {
            return Err(ServerResponseError::NotFound);
        };
        let update_data = data.into_inner();

        update_user_data(&state.db, user_id, update_data).await?;
        Ok(HttpResponse::Ok().finish())
    }
}
