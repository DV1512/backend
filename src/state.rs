use crate::server_error::ServerError;
use actix_web::web;
use std::sync::Arc;
use surrealdb::engine::local::Db;
use surrealdb::{engine::local::RocksDb, Connection, Surreal};

pub struct AppState<C: Connection> {
    pub db: Arc<Surreal<C>>,
}

pub async fn db(ns: &str, db: &str) -> Result<Surreal<Db>, ServerError> {
    let database = Surreal::new::<RocksDb>(std::env::temp_dir().join("temp.db")).await?;

    database.use_ns(ns).use_db(db).await?;

    Ok(database)
}

pub async fn app_state() -> Result<web::Data<AppState<Db>>, ServerError> {
    let database = db("default", "default").await?;

    Ok(web::Data::new(AppState {
        db: Arc::new(database),
    }))
}
