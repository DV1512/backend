use std::sync::Arc;
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use surrealdb::Surreal;
use tracing::info;
use crate::error::ServerResponseError;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) struct AuthenticatedUser {
    pub(crate) email: String,
    pub(crate) id: Thing,
    username: String,
}

/// Validates a given username and password,
/// returning ``Ok(AuthenticatedUser)`` for valid credentials
/// and ``Err(ServerResponse::UnauthorizedWithMessage)``
/// otherwise.
pub(crate) async fn validate_user<T>(
    username: String,
    password: String,
    db: &Arc<Surreal<T>>,
) -> Result<AuthenticatedUser, ServerResponseError>
where
    T: surrealdb::Connection,
{
    let password: surrealdb::sql::Value = password.into();
    let query = format!(
        "SELECT * FROM user
        WHERE email = $email
        AND array::any(
            <-auth_for<-user_auth, |$a|
            type::is::string($a.password) AND
            crypto::argon2::compare($a.password, {})
        )
        FETCH auth;",
        password
    );
    info!("DB query: {query}");

    let query_result: Option<AuthenticatedUser> =
        db.query(query).bind(("email", username)).await?.take(0)?;
    info!("Query result: {:?}", &query_result);

    query_result.ok_or(ServerResponseError::UnauthorizedWithMessage(
        "Invalid username or password".to_string(),
    ))
}