use crate::auth::users::auth::UserAuth;
use crate::auth::users::get::utils::get_user_by_token;
use crate::auth::UserInfo;
use crate::error::ServerResponseError;
use crate::extractors::{Auth, IntoSession};
use crate::generate_endpoint;
use crate::state::AppState;
use actix_web::{web, Either, HttpResponse};
use utoipa::ToSchema;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserUpdateRequest {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Password {
    data: String,
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
        token: Auth,
        state: web::Data<AppState>,
    };
    {
        let session = match token {
            Either::Left(identity) => {
                let session = identity.get_session().await;

                if session.is_none() {
                    return Err(ServerResponseError::Unauthorized);
                }

                session.unwrap()
            },
            Either::Right(session) => {
                session
            },
        };
        let update_data = data.into_inner();
        let user = get_user_by_token(&state.db, &session.access_token).await?;
        dbg!(&user);

        // This unwrap is ugly!
        // A 'UserInfo' returned from the database should have a user ID field with type Thing
        // and not type Option<Thing>
        let user_id = user.id.unwrap();
        dbg!(&user_id);

        let updated_user: Option<UserInfo> = state.db.query("UPDATE $user_id MERGE $user_data")
            .bind(("user_id", user_id.clone()))
            .bind(("user_data", update_data.clone())).await?.take(0)?;
        dbg!(updated_user);

        if let Some(password) = update_data.password {
            // I don't like that I have to use "CONTAINS" here, can probably be improved.
            let updated_user_auth: Option<UserAuth> = state.db
                .query("UPDATE user_auth SET password = $new_password WHERE ->auth_for->user.id CONTAINS $user_id;")
                .bind(("new_password", password))
                .bind(("user_id", user_id)).await?.take(0)?;
            dbg!(updated_user_auth);
        }

        Ok(HttpResponse::Ok().finish())
    }
}
