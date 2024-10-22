pub(crate) mod utils;

use crate::auth::users::get::utils::get_user_by_token;
use crate::auth::UserInfo;
use crate::dto::UserInfoDTO;
use crate::extractors::Auth;
use crate::AppState;
use actix_web::{web, HttpResponse, Responder};
use anyhow::Result;
use helper_macros::generate_endpoint;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use surrealdb::Surreal;
use tosic_utils::{Select, Statement};
use tracing::{error, info};
use utoipa::{IntoParams, ToSchema};

#[tracing::instrument(skip(db))]
pub async fn get_user_by_username<T>(db: &Arc<Surreal<T>>, username: &str) -> Result<UserInfo>
where
    T: surrealdb::Connection,
{
    let query = Select::query("user")
        .add_condition("url_safe_username", None, username)
        .set_limit(1);

    let user: Option<UserInfo> = query.run(db, 0).await?;

    if let Some(user) = user {
        info!("User found: {:?}", user);
        Ok(user)
    } else {
        info!("User not found");
        Err(anyhow::anyhow!("User not found"))
    }
}

#[tracing::instrument(skip(db, email))]
pub(crate) async fn get_user_by_email<T>(db: &Arc<Surreal<T>>, email: &str) -> Result<UserInfo>
where
    T: surrealdb::Connection,
{
    let query = Select::query("user")
        .add_condition("email", None, email)
        .set_limit(1);

    let user: Option<UserInfo> = query.run(db, 0).await?;

    if let Some(user) = user {
        Ok(user)
    } else {
        Err(anyhow::anyhow!("User not found"))
    }
}

#[tracing::instrument(skip(db, email))]
pub(crate) async fn get_user<T>(db: &Arc<Surreal<T>>, email: &str) -> Option<UserInfo>
where
    T: surrealdb::Connection,
{
    get_user_by_email(db, email).await.ok()
}

#[derive(Serialize, Deserialize, Debug, Clone, IntoParams, ToSchema)]
pub struct GetUserBy {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[param(example = "johndoe")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[param(example = "johndoe@example.com")]
    pub email: Option<String>,
    #[param(example = "token123")]
    pub token: Option<String>,
}

#[tracing::instrument(skip(db, data))]
pub(crate) async fn get_user_by_internal<T>(
    db: &Arc<Surreal<T>>,
    data: &GetUserBy,
) -> Result<impl Responder, ServerResponseError>
where
    T: surrealdb::Connection,
{
    if data.email.is_none() && data.token.is_none() && data.username.is_none() {
        return Err(ServerResponseError::BadRequest(
            "No data provided".to_string(),
        ));
    }

    if let Some(email) = &data.email {
        let user = match get_user_by_email(db, email).await {
            Ok(user) => user,
            Err(e) => {
                error!("Failed to get user by email: {:?}", e);

                return Err(ServerResponseError::NotFound);
            }
        };

        let user_dto: UserInfoDTO = user.into();
        return Ok(HttpResponse::Ok().json(user_dto));
    }

    if let Some(username) = &data.username {
        let user = match get_user_by_username(db, username).await {
            Ok(user) => user,
            Err(e) => {
                error!("Failed to get user by username: {:?}", e);

                return Err(ServerResponseError::NotFound);
            }
        };

        let user_dto: UserInfoDTO = user.into();
        return Ok(HttpResponse::Ok().json(user_dto));
    }

    if let Some(token) = &data.token {
        let user = match get_user_by_token(db, token).await {
            Ok(user) => user,
            Err(e) => {
                error!("Failed to get user by token: {:?}", e);

                return Err(ServerResponseError::NotFound);
            }
        };

        let user_dto: UserInfoDTO = user.into();
        Ok(HttpResponse::Ok().json(user_dto))
    } else {
        Err(ServerResponseError::BadRequest(
            "No data provided".to_string(),
        ))
    }
}

use crate::auth::UserInfoExampleResponses;
use crate::error::ServerResponseError;

generate_endpoint! {
    fn get_user_by;
    method: get;
    path: "";
    docs: {
        params: (GetUserBy),
        tag: "user",
        responses: {
            (status = 200, response = UserInfoExampleResponses),
            (status = 404, description = "User not found"),
        },
        security: [
            ("bearer_token" = []),
            ("cookie_session" = []),
        ]
    }
    params: {
        _auth: Auth,
        state: web::Data<AppState>,
        data: web::Query<GetUserBy>,
    };
    {
        info!("Retrieving user");
        let data = data.into_inner();
        let db = &state.db;
        get_user_by_internal(db, &data).await
    }
}
