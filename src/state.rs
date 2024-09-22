use crate::auth::oauth::Oauth;
use crate::server_error::ServerError;
use crate::INTERNAL_DB;
use actix_web::web;
use std::sync::Arc;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::Surreal;

pub struct AppState {
    pub db: Arc<Surreal<Client>>,
    pub oauth: Arc<Oauth>,
}

#[tracing::instrument]
async fn setup_db() -> Result<(), ServerError> {
    let db_url = tosic_utils::prelude::env!("SURREALDB_URL");
    let db_user = tosic_utils::prelude::env!("SURREALDB_USER");
    let db_pass = tosic_utils::prelude::env!("SURREALDB_PASS");

    INTERNAL_DB.connect::<Ws>(db_url).await?;

    INTERNAL_DB
        .signin(surrealdb::opt::auth::Root {
            username: db_user.as_str(),
            password: db_pass.as_str(),
        })
        .await?;

    let namespace = tosic_utils::prelude::env!("SURREALDB_NAMESPACE", "default");
    let database = tosic_utils::prelude::env!("SURREALDB_DATABASE", "default");

    INTERNAL_DB
        .use_ns(namespace.as_str())
        .use_db(database.as_str())
        .await?;

    Ok(())
}

#[tracing::instrument]
pub async fn db(ns: &str, db: &str) -> Result<Surreal<Client>, ServerError> {
    let db_url = tosic_utils::prelude::env!("SURREALDB_URL");
    let db_user = tosic_utils::prelude::env!("SURREALDB_USER");
    let db_pass = tosic_utils::prelude::env!("SURREALDB_PASS");

    let database = Surreal::new::<Ws>(db_url).await?;

    database
        .signin(surrealdb::opt::auth::Root {
            username: &db_user,
            password: &db_pass,
        })
        .await?;

    database.use_ns(ns).use_db(db).await?;

    Ok(database)
}

#[tracing::instrument]
pub async fn app_state() -> Result<web::Data<AppState>, ServerError> {
    let database = db("default", "default").await?;

    Ok(web::Data::new(AppState {
        db: Arc::new(database),
        oauth: Arc::new(Oauth::new().await?),
    }))
}
