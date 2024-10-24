pub(crate) mod utils;

use crate::auth::users::get::utils::get_user_by_token;
use crate::auth::UserInfo;
use crate::dto::UserInfoDTO;
use crate::extractors::Auth;
use crate::AppState;
use actix_web::web;
use anyhow::Result;
use helper_macros::generate_endpoint;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use surrealdb::Surreal;
use tosic_utils::{Select, Statement};
use tracing::{info, warn};
use utoipa::openapi::path::{Parameter, ParameterBuilder, ParameterIn};
use utoipa::openapi::{KnownFormat, Object, ObjectBuilder, SchemaFormat, Type};
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
        warn!("User not found");
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

#[tracing::instrument(skip(db, data))]
pub(crate) async fn get_user_by_internal<T>(
    db: &Arc<Surreal<T>>,
    data: &GetUserBy,
) -> Result<UserInfoDTO, ServerResponseError>
where
    T: surrealdb::Connection,
{
    match data {
        GetUserBy::Email { email } => get_user_by_email(db, email).await,
        GetUserBy::Username { username } => get_user_by_username(db, username).await,
        GetUserBy::Token { token } => get_user_by_token(db, token).await,
    }
    .map_err(|_| ServerResponseError::NotFound)
    .map(|user| user.into())
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(untagged)]
pub(crate) enum GetUserBy {
    Username { username: String },
    Email { email: String },
    Token { token: String },
}

impl IntoParams for GetUserBy {
    fn into_params(parameter_in_provider: impl Fn() -> Option<ParameterIn>) -> Vec<Parameter> {
        let parameter_in = parameter_in_provider().unwrap_or(ParameterIn::Query);
        let mut params = vec![ParameterBuilder::new()
            .name("username")
            .description(Some("User's username"))
            .schema::<Object>(Some(ObjectBuilder::new().schema_type(Type::String).build()))
            .parameter_in(parameter_in.clone())
            .build()];

        params.push(
            ParameterBuilder::new()
                .name("email")
                .description(Some("User's email"))
                .schema::<Object>(Some(
                    ObjectBuilder::new()
                        .schema_type(Type::String)
                        .format(Some(SchemaFormat::KnownFormat(KnownFormat::Email)))
                        .build(),
                ))
                .parameter_in(parameter_in.clone())
                .build(),
        );

        params.push(
            ParameterBuilder::new()
                .name("token")
                .description(Some("User's token"))
                .schema::<Object>(Some(ObjectBuilder::new().schema_type(Type::String).build()))
                .parameter_in(parameter_in)
                .build(),
        );

        params
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
            (status = 401, description = "Invalid credentials"),
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
        let user = get_user_by_internal(db, &data).await?;

        Ok(web::Json(user))
    }
}
