use crate::auth::{session::UserSessionWithInfo, UserInfo, Users};
use crate::{CountResponse, PaginationResponse};
use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use surrealdb::Surreal;
use tokio::try_join;
use tosic_utils::{Filter, QueryBuilder, Select, Statement};
use tracing::error;

pub(crate) async fn get_user_by_token<T>(db: &Arc<Surreal<T>>, token: &str) -> Result<UserInfo>
where
    T: surrealdb::Connection,
{
    let query = Select::query("session").add_field("user_id.*", Some("user")).add_field("*", None).add_condition("access_token", None, token).set_limit(1);

    let user: Option<UserSessionWithInfo> = query.run(db, 0).await?;

    if let Some(user) = user {
        Ok(user.user.expect("User not found"))
    } else {
        Err(anyhow::anyhow!("User not found"))
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub(crate) struct GetUserByFilter {
    #[serde(skip_serializing_if = "Option::is_none")]
    email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    url_safe_username: Option<String>,
    /*#[serde(flatten)]
    pagination: PaginationRequest*/
    limit: Option<u64>,
    offset: Option<u64>,
}

impl From<GetUserByFilter> for Filter {
    fn from(filter: GetUserByFilter) -> Self {
        let mut new_filter = Filter::default();

        if let Some(email) = filter.email {
            new_filter = new_filter.add_condition("email".to_string(), None, email);
        }

        if let Some(username) = filter.username {
            new_filter = new_filter.add_condition("username".to_string(), None, username);
        }

        if let Some(user_id) = filter.user_id {
            new_filter = new_filter.add_condition("user_id".to_string(), None, user_id);
        }

        if let Some(url_safe_username) = filter.url_safe_username {
            new_filter =
                new_filter.add_condition("url_safe_username".to_string(), None, url_safe_username);
        }

        new_filter
    }
}

#[allow(dead_code)]
fn construct_basic_query(filter: &GetUserByFilter) -> (QueryBuilder<Select>, QueryBuilder<Select>) {
    let limit = filter.limit.unwrap_or(10);
    let offset = filter.offset.unwrap_or(0);
    let filter = Filter::from(filter.clone());

    let sql = Select::query("user")
        .set_filter(filter.clone())
        .set_limit(limit)
        .set_start(offset);
    let count = Select::query("user")
        .set_filter(filter)
        .add_field("count()", Some("count"))
        .group_all();

    (sql, count)
}

#[allow(dead_code)]
pub(crate) async fn get_users_by_filter<T>(
    db: &Arc<Surreal<T>>,
    filter: GetUserByFilter,
) -> Result<Users>
where
    T: surrealdb::Connection,
{
    let limit = filter.limit.unwrap_or(10);
    let offset = filter.offset.unwrap_or(0);
    let (sql, count) = construct_basic_query(&filter);

    let users = sql.run(db, 0);
    let count = count.run(db, 0);

    let (users, total): (Vec<UserInfo>, Option<CountResponse>) = match try_join!(users, count) {
        Ok((users, count)) => (users, count),
        Err(e) => {
            error!("Error getting users: {}", e);
            return Err(anyhow::anyhow!("Error getting users"));
        }
    };

    let total = total.unwrap_or(CountResponse { count: 0 });

    let pagination = PaginationResponse {
        total: Some(total.count),
        limit: Some(limit),
        offset: Some(offset),
    };

    let response = Users { users, pagination };

    Ok(response)
}
