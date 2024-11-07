use crate::auth::oauth::url_safe_string;
use crate::auth::users::get::utils::get_user_by_token;
use crate::error::ServerResponseError;
use crate::extractors::AuthenticatedToken;
use crate::extractors::IntoSession;
use crate::generate_endpoint;
use crate::models::thing::Thing;
use crate::state::AppState;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use surrealdb::Surreal;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserUpdateRequest {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub password: Option<String>,
}

pub async fn update_user_data<T>(
    db: &Arc<Surreal<T>>,
    user_id: Thing,
    update_data: UserUpdateRequest,
) -> Result<(), ServerResponseError>
where
    T: surrealdb::Connection,
{
    let url_safe_username = url_safe_string(&update_data.username);
    const SQL: &str = "UPDATE $user_id SET
		username = $username,
		url_safe_username = $url_safe_username,
		first_name = $first_name,
		last_name = $last_name;";
    db.query(SQL)
        .bind(("user_id", user_id.clone()))
        .bind(("username", update_data.username))
        .bind(("url_safe_username", url_safe_username))
        .bind(("first_name", update_data.first_name))
        .bind(("last_name", update_data.last_name))
        .await?;

    if let Some(password) = update_data.password {
        const SQL: &str = "UPDATE user_auth SET password = $new_password WHERE ->auth_for->user.id CONTAINS $user_id;";
        db.query(SQL)
            .bind(("new_password", password))
            .bind(("user_id", user_id))
            .await?;
    }
    Ok(())
}

generate_endpoint! {
    fn user_update_endpoint;
    method: put;
    path: "/update";
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
