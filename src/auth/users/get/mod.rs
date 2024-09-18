pub(crate) mod utils;

use crate::auth::users::get::utils::{get_user_by_token, GetUserByFilter};
use crate::auth::UserInfo;
use crate::AppState;
use actix_web::{get, web, HttpResponse, Responder};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use surrealdb::engine::local::Db;
use surrealdb::Surreal;
use tracing::{error, info};

pub async fn get_user_by_username<T>(db: &Arc<Surreal<T>>, username: &str) -> Result<UserInfo>
where
    T: surrealdb::Connection,
{
    let sql = "SELECT * FROM type::table($table) WHERE url_safe_username = $username LIMIT 1";

    let mut res = db
        .query(sql)
        .bind(("table", "user"))
        .bind(("username", username))
        .await?;

    info!("Query result: {:?}", res);

    let user: Option<UserInfo> = res.take(0)?;

    if let Some(user) = user {
        info!("User found: {:?}", user);
        Ok(user)
    } else {
        info!("User not found");
        Err(anyhow::anyhow!("User not found"))
    }
}

pub(crate) async fn get_user_by_email<T>(db: &Arc<Surreal<T>>, email: &str) -> Result<UserInfo>
where
    T: surrealdb::Connection,
{
    let sql = "SELECT * FROM type::table($table) WHERE email = $email LIMIT 1";

    let mut res = db
        .query(sql)
        .bind(("table", "user"))
        .bind(("email", email))
        .await?;

    let user: Option<UserInfo> = res.take(0)?;

    if let Some(user) = user {
        Ok(user)
    } else {
        Err(anyhow::anyhow!("User not found"))
    }
}

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
    state: web::Data<AppState<Db>>,
) -> impl Responder {
    let data = data.into_inner();
    let db = &state.db;
    get_user_by_internal(db, &data).await
}
