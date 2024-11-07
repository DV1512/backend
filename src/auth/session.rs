use crate::auth::UserInfo;
use crate::server::db::INTERNAL_DB;
use anyhow::{bail, Result};
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::{Datetime, Thing};
use tosic_utils::filter::Filter;
use tosic_utils::query::delete::Delete;
use tosic_utils::query::select::Select;
use tosic_utils::query::{Query, Statement};
use tosic_utils::Create;
use tracing::error;

#[derive(Debug, Serialize, Deserialize, PartialOrd, Eq, PartialEq, Clone)]
pub(crate) struct UserSession {
    pub(crate) email: String,
    pub(crate) access_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) refresh_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) created_at: Option<Datetime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) expires_at: Option<Datetime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) id: Option<Thing>,
    pub(crate) user_id: Thing,
}

#[derive(Debug, Serialize, Deserialize, PartialOrd, Eq, PartialEq, Clone)]
pub(crate) struct UserSessionWithInfo {
    #[serde(flatten)]
    pub(crate) session: UserSession,
    pub(crate) user: Option<UserInfo>,
}

impl UserSessionWithInfo {
    #[allow(dead_code)]
    pub(crate) fn new(session: UserSession, user: Option<UserInfo>) -> Self {
        Self { session, user }
    }

    #[allow(dead_code)]
    #[tracing::instrument]
    async fn fetch_with_filter(filter: Filter) -> Option<Self> {
        Select::query("session")
            .add_field("user_id.*", Some("user"))
            .set_omit_field("user_id")
            .set_filter(filter)
            .set_limit(1)
            .run_lazy(&INTERNAL_DB, 0)
            .await
            .unwrap_or_else(|e| {
                error!("Error fetching session: {}", e);
                None
            })
    }

    #[allow(dead_code)]
    #[tracing::instrument]
    pub(crate) async fn fetch_by_email(email: String) -> Option<Self> {
        let filter = Filter::default()
            .add_condition("email".to_string(), None, email)
            .add_condition(
                "expires_at".to_string(),
                Some(">".to_string()),
                Datetime::default(),
            );

        Self::fetch_with_filter(filter).await
    }
}

#[allow(dead_code)]
impl UserSession {
    const CREATE: &'static str = "CREATE session set email = $email, access_token = $access_token, refresh_token = $refresh_token, user_id = $user_id";

    const UPDATE: &'static str = "UPDATE session MERGE { access_token: $access_token, refresh_token: $refresh_token, expires_at: time::now() + 1h }";

    const DELETE: &'static str = "DELETE session";

    const SELECT: &'static str = "SELECT * FROM session";

    pub(crate) fn new(
        access_token: String,
        refresh_token: Option<String>,
        email: String,
        user_id: Thing,
    ) -> Self {
        Self {
            email,
            access_token,
            refresh_token,
            created_at: None,
            expires_at: None,
            id: None,
            user_id,
        }
    }

    /// Update the session to reflect a new access token, refresh token, and expiration time
    #[tracing::instrument]
    pub(crate) async fn update(self) -> Result<Self> {
        let sql = "UPDATE session MERGE { access_token: $access_token, refresh_token: $refresh_token, expires_at: time::now() + 1h } WHERE email = $email";

        let mut res = INTERNAL_DB
            .query(sql)
            .bind(("access_token", self.access_token))
            .bind(("refresh_token", self.refresh_token))
            .bind(("email", self.email))
            .await?;

        let sessions: Option<Self> = res.take(0)?;

        if let Some(session) = sessions {
            Ok(session)
        } else {
            bail!("Error updating session")
        }
    }

    #[tracing::instrument]
    pub(crate) async fn create(self) -> Result<Self> {
        let query = Create::query("session")
            .add_field_to_content("access_token", self.access_token)
            .add_field_to_content("refresh_token", self.refresh_token)
            .add_field_to_content("user_id", self.user_id)
            .add_field_to_content("email", self.email)
            .add_field_to_content("expires_at", Utc::now() + Duration::hours(1))
            .add_field_to_content("created_at", Utc::now());

        let sessions: Option<Self> = query.run_lazy(&INTERNAL_DB, 0).await?;

        if let Some(session) = sessions {
            Ok(session)
        } else {
            bail!("Error creating session")
        }
    }

    #[tracing::instrument]
    pub(crate) async fn fetch() -> Result<Vec<Self>> {
        let sql = Select::query("session");

        let sessions: Vec<Self> = sql.run_lazy(&INTERNAL_DB, 0).await?;

        Ok(sessions)
    }

    #[tracing::instrument]
    pub(crate) async fn fetch_by_id(id: Thing) -> Option<Self> {
        let query = Select::query("session")
            .add_condition("id", None, id)
            .add_condition("expires_at", Some(">"), Datetime::default())
            .set_limit(1);

        let session: Option<Self> = match query.run_lazy(&INTERNAL_DB, 0).await {
            Ok(session) => session,
            Err(_) => return None,
        };

        session
    }

    #[tracing::instrument]
    pub(crate) async fn fetch_by_access_token(access_token: String) -> Option<Self> {
        let query = Select::query("session")
            .add_condition("access_token", None, access_token)
            .add_condition("expires_at", Some(">"), Datetime::default())
            .set_limit(1);

        let session: Option<Self> = match query.run_lazy(&INTERNAL_DB, 0).await {
            Ok(session) => session,
            Err(_) => return None,
        };

        session
    }

    #[tracing::instrument]
    async fn fetch_with_filter(filter: Filter) -> Option<Self> {
        let sql = Select::query("session").set_filter(filter).set_limit(1);

        Self::delete_expired()
            .await
            .expect("Error deleting expired sessions");

        sql.run_lazy(&INTERNAL_DB, 0).await.unwrap_or_else(|e| {
            error!("Error fetching session: {}", e);
            None
        })
    }

    #[tracing::instrument]
    pub(crate) async fn fetch_by_email(email: String) -> Option<Self> {
        let filter = Filter::default()
            .add_condition("email".to_string(), None, email)
            .add_condition(
                "expires_at".to_string(),
                Some(">".to_string()),
                Datetime::default(),
            );

        let session: Option<Self> = Self::fetch_with_filter(filter).await;

        session
    }

    #[tracing::instrument]
    pub(crate) async fn fetch_by_user_id(user_id: Thing) -> Option<Self> {
        let sql = Filter::default()
            .add_condition("user_id".to_string(), None, user_id)
            .add_condition(
                "expires_at".to_string(),
                Some(">".to_string()),
                "time::now()",
            );

        let session: Option<Self> = Self::fetch_with_filter(sql).await;

        session
    }

    #[tracing::instrument]
    pub(crate) async fn delete(self) -> Result<()> {
        let sql = Delete::query("session")
            .add_condition("email", None, self.email)
            .construct();

        INTERNAL_DB.query(sql).await?;
        Ok(())
    }

    #[tracing::instrument]
    pub async fn delete_expired() -> Result<()> {
        let query =
            Delete::query("session").add_condition("expires_at", Some("<"), Datetime::default());

        INTERNAL_DB.query(query.construct()).await?;

        Ok(())
    }
}
