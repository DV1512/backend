pub(crate) mod utils;

use crate::auth::users::get::utils::get_user_by_token;
use crate::auth::UserInfo;
use crate::AppState;
use actix_web::{get, web, HttpResponse, Responder};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use surrealdb::Surreal;
use tosic_utils::{Select, Statement};
use tracing::{error, info};

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetUserBy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

#[tracing::instrument(skip(db, data))]
pub(crate) async fn get_user_by_internal<T>(
    db: &Arc<Surreal<T>>,
    data: &GetUserBy,
) -> impl Responder
where
    T: surrealdb::Connection,
{
    if data.email.is_none() && data.token.is_none() && data.username.is_none() {
        return HttpResponse::BadRequest().body("Missing email or token");
    }

    // if email is present then we use the email to get the user and not the token
    if let Some(email) = &data.email {
        let user = match get_user_by_email(db, email).await {
            Ok(user) => user,
            Err(e) => {
                error!("Failed to get user by email: {:?}", e);
                return HttpResponse::NotFound().body("User not found");
            }
        };

        return HttpResponse::Ok().json(user);
    }

    if let Some(username) = &data.username {
        let user = match get_user_by_username(db, username).await {
            Ok(user) => user,
            Err(e) => {
                error!("Failed to get user by username: {:?}", e);
                return HttpResponse::NotFound().body("User not found");
            }
        };

        return HttpResponse::Ok().json(user);
    }

    if let Some(token) = &data.token {
        let user = match get_user_by_token(db, token).await {
            Ok(user) => user,
            Err(e) => {
                error!("Failed to get user by token: {:?}", e);
                return HttpResponse::NotFound().body("User not found");
            }
        };

        HttpResponse::Ok().json(user)
    } else {
        HttpResponse::BadRequest().body("Missing email or token")
    }
}

#[get("/by")]
pub(crate) async fn get_user_by(
    data: web::Query<GetUserBy>,
    state: web::Data<AppState>,
) -> impl Responder {
    let data = data.into_inner();
    let db = &state.db;
    get_user_by_internal(db, &data).await
}
