use crate::error::ServerResponseError;
use crate::models::thing::Thing;
use std::sync::Arc;
use surrealdb::Surreal;

pub async fn delete_user<T>(db: &Arc<Surreal<T>>, user_id: Thing) -> Result<(), ServerResponseError>
where
    T: surrealdb::Connection,
{
    const SQL: &str = "
            BEGIN TRANSACTION;
            DELETE user_auth WHERE ->auth_for->user.id CONTAINS $USER_ID;
            DELETE $USER_ID;
            COMMIT TRANSACTION;
        ";
    db.query(SQL).bind(("USER_ID", user_id)).await?;

    Ok(())
}
