use crate::auth::UserInfo;
use crate::Record;
use crate::{auth::oauth::register::UserRegistrationRequest, error::ServerResponseError};
use anyhow::{bail, Result};
use std::sync::Arc;
use surrealdb::Surreal;

#[tracing::instrument(skip(db, user))]
pub async fn create_user<T>(db: &Arc<Surreal<T>>, user: UserInfo) -> Result<Record>
where
    T: surrealdb::Connection,
{
    let sql = "CREATE user SET username = $username, first_name = $first_name, last_name = $last_name, email = $email, picture = $picture, role = $role";

    let mut res = db
        .query(sql)
        .bind(("username", user.username.clone()))
        .bind(("first_name", user.first_name.clone()))
        .bind(("last_name", user.last_name.clone()))
        .bind(("email", user.email.clone()))
        .bind(("picture", user.picture.clone()))
        .bind(("role", user.role.clone()))
        .await?;

    let records: Vec<Record> = res.take(0)?;

    let record = if let Some(record) = records.into_iter().next() {
        record
    } else {
        bail!("Error creating user");
    };

    Ok(record)
}

#[tracing::instrument(skip(db, user_registration))]
pub async fn register_user<T>(
    db: &Arc<Surreal<T>>,
    user_registration: UserRegistrationRequest,
) -> Result<(), ServerResponseError>
where
    T: surrealdb::Connection,
{
    let password = user_registration.password.clone();

    const REGISTER_USER_SQL: &str = "
        BEGIN TRANSACTION;

        LET $USER = (
            CREATE user CONTENT $user_content
        );

        LET $USER_AUTH = (
            CREATE user_auth SET 
            providers = [ provider:Email ],
            password = $password
        );

        RELATE ($USER_AUTH) -> auth_for -> ($USER);

        COMMIT TRANSACTION;
    ";

    db.query(REGISTER_USER_SQL)
        .bind(("user_content", user_registration))
        .bind(("password", password))
        .await?
        .check()?;
    Ok(())
}
