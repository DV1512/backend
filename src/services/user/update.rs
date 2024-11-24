use crate::dto::user_update_request::UserUpdateRequest;
use crate::error::ServerResponseError;
use crate::models::thing::Thing;
use std::sync::Arc;
use surrealdb::Surreal;

pub async fn update_user_data<T>(
    db: &Arc<Surreal<T>>,
    user_id: Thing,
    update_data: UserUpdateRequest,
) -> Result<(), ServerResponseError>
where
    T: surrealdb::Connection,
{
    const SQL: &str = "UPDATE $user_id SET
		username = $username,
		url_safe_username = $url_safe_username,
		first_name = $first_name,
		last_name = $last_name;";
    db.query(SQL)
        .bind(("user_id", user_id.clone()))
        .bind(("username", update_data.username))
        .bind(("first_name", update_data.first_name))
        .bind(("last_name", update_data.last_name))
        .await?;

    if let Some(password) = update_data.password {
        const SQL: &str = "UPDATE user_auth SET password = $new_password WHERE ->auth_for->user.id CONTAINS $user_id;";
        db.query(SQL)
            .bind(("new_password", password))
            .bind(("user_id", user_id))
            .await?;
    }
    Ok(())
}
