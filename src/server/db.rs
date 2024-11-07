use surrealdb::engine::remote::ws::{Client, Ws};
use once_cell::sync::Lazy;
use surrealdb::Surreal;
use crate::server_error::ServerError;

pub(crate) static INTERNAL_DB: Lazy<Surreal<Client>> = Lazy::new(Surreal::init);

#[tracing::instrument]
pub(crate) async fn init_internal_db() -> Result<(), ServerError> {
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