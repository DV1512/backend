use crate::auth::oauth::provider::OauthProvider;
use surrealdb::sql::Datetime;

#[allow(dead_code)]
pub(crate) struct UserAuth {
    providers: Vec<OauthProvider>,
    created_at: Datetime,
    updated_at: Datetime,
    password: Option<String>,
}
