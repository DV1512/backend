use crate::auth::oauth::register::UserRegistrationRequest;
use crate::auth::UserInfo;
use crate::Record;
use anyhow::{bail, Result};
use std::sync::Arc;
use surrealdb::Surreal;

#[tracing::instrument(skip(db, user))]
pub async fn create_user<T>(db: &Arc<Surreal<T>>, user: UserInfo) -> Result<Record>
where
    T: surrealdb::Connection,
{
    let sql = "CREATE user SET username = $username, url_safe_username = $url_safe_username, first_name = $first_name, last_name = $last_name, email = $email, picture = $picture, role = $role";

    let mut res = db
        .query(sql)
        .bind(("username", user.username.clone()))
        .bind(("url_safe_username", user.url_safe_username.clone()))
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

pub async fn register_user<T>(
    db: &Arc<Surreal<T>>,
    user_registration: UserRegistrationRequest,
) -> Result<()>
where
    T: surrealdb::Connection,
{
    let password = user_registration.password.clone();
    let user = UserInfo::from(user_registration);

    const REGISTER_USER_SQL: &str = "
        BEGIN TRANSACTION;

        LET $USER = (
            CREATE user SET
            username = $username,
            url_safe_username = $url_safe_username,
            first_name = $first_name,
            last_name = $last_name,
            email = $email,
            picture = $picture,
            role = $role
        );

        LET $USER_AUTH = (
            CREATE user_auth SET 
            providers = [ provider:Email ],
            password = crypto::argon2::generate($password)
        );

        RELATE ($USER_AUTH.id) -> auth_for -> ($USER.id);

        COMMIT TRANSACTION;
    ";

    let full_query = db
        .query(REGISTER_USER_SQL)
        .bind(("username", user.username))
        .bind(("url_safe_username", user.url_safe_username))
        .bind(("first_name", user.first_name))
        .bind(("last_name", user.last_name))
        .bind(("email", user.email))
        .bind(("picture", user.picture))
        .bind(("role", user.role))
        .bind(("password", password));

    if let Err(err) = full_query.await?.check() {
        bail!("Could not register user: '{err}'");
    }
    Ok(())
}
