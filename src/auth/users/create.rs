use anyhow::{bail, Result};
use std::sync::Arc;
use surrealdb::Surreal;

use crate::auth::UserInfo;
use crate::Record;

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
