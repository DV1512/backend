use anyhow::bail;
use crate::auth::oauth::provider::{OauthProvider, OauthProviderName};
use crate::models::auth_for::AuthForRelation;
use crate::models::Record;
use crate::server::db::INTERNAL_DB;

#[tracing::instrument(skip(password))]
pub(crate) async fn create_auth_for_user(
    user_id: Record,
    providers: Vec<OauthProvider>,
    password: Option<String>,
) -> anyhow::Result<()> {
    let require_password = providers.iter().any(|p| p.name == OauthProviderName::Email);
    let providers_ids = providers.iter().map(|p| p.id.clone()).collect::<Vec<_>>();
    let user_id = user_id.id;

    if require_password && password.is_none() {
        bail!("User requires password. Please provide one.");
    }

    let mut res = if password.is_some() {
        let sql = "CREATE user_auth set providers = $providers, password = $password";

        INTERNAL_DB
            .query(sql)
            .bind(("providers", providers_ids))
            .bind(("password", password))
            .await?
    } else {
        let sql = "CREATE user_auth set providers = $providers";

        INTERNAL_DB
            .query(sql)
            .bind(("providers", providers_ids))
            .await?
    };

    let user_auth: Option<AuthForRelation> = res.take(0)?;

    if let Some(user_auth) = user_auth {
        let query = "RELATE $user_auth->auth_for->$user_id";
        INTERNAL_DB
            .query(query)
            .bind(("user_auth", user_auth.id))
            .bind(("user_id", user_id))
            .await?;
    } else {
        bail!("Error creating user_auth");
    }

    Ok(())
}