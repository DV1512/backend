use crate::auth::users::get::utils::{get_users_by_filter, GetUserByFilter};
use crate::auth::Users;
use crate::state::AppState;
use actix_web::{web, HttpResponse};
use async_graphql::{EmptyMutation, EmptySubscription, Object, Schema};
use async_graphql_actix_web::GraphQLRequest;
use helper_macros::generate_endpoint;
use std::sync::Arc;
use surrealdb::Surreal;

pub(crate) struct Query<T: surrealdb::Connection> {
    db: Arc<Surreal<T>>,
}

impl<T: surrealdb::Connection> Query<T> {
    pub fn new(db: Arc<Surreal<T>>) -> Self {
        Self { db }
    }
}

#[Object]
impl<T: surrealdb::Connection> Query<T> {
    async fn get_users(&self) -> async_graphql::Result<Users> {
        match get_users_by_filter(
            &self.db,
            GetUserByFilter {
                ..Default::default()
            },
        )
        .await
        {
            Ok(users) => Ok(users),
            Err(e) => Err(async_graphql::Error::new(e.to_string())),
        }
    }
}

generate_endpoint! {
    fn graphql_test;
    method: post;
    path: "/graphql-test";
    docs: {
        responses: {
            (status = 200, description = "Successful GraphQL request"),
            (status = 401, description = "Invalid credentials"),
            (status = 500, description = "Something happened when forwarding the GraphQL request"),
        }
    }
    params: {
        request: GraphQLRequest,
        state: web::Data<AppState>,
    }
    {
        let db = &state.db;
        let schema = Schema::new(Query::new(db.clone()), EmptyMutation, EmptySubscription);

        let response = schema.execute(request.into_inner()).await;

        Ok(HttpResponse::Ok().json(response))
    }
}
