use actix_web::{web, HttpRequest, HttpResponse};
use helper_macros::generate_endpoint;
use crate::auth::UserInfo;
use crate::dto::access_token_request::{AccessTokenRequestDTO, GrantType};
use crate::error::ServerResponseError;
use crate::state::AppState;

generate_endpoint! {
    fn token;
    method: post;
    path: "/token";
    params: {
        state: web::Data<AppState>,
        query: web::Query<AccessTokenRequestDTO>,
        req: HttpRequest,
    }
    {
        let state = state.clone();
        let query = query.into_inner();
        let db = state.db.clone();

        if query.grant_type != GrantType::Password {
            return Err(ServerResponseError::BadRequest("Invalid grant type".to_string()));
        }

        let db_query = format!("SELECT *, <-auth_for<-user_auth AS auth FROM user WHERE email = \"{}\" AND array::any(<-auth_for<-user_auth, |$a| !type::is::none($a.password) AND type::is::string($a.password) AND crypto::argon2::compare($a.password, \"{}\")) FETCH auth;", query.username, query.password);

        let user: Option<UserInfo> = db
            .query(db_query)
            .await?
            .take(0)?;

        if let Some(user) = user {
            dbg!(&user);
            Ok(HttpResponse::Ok().json(user))
        } else {
            Err(ServerResponseError::BadRequest("You are not allowed in".to_string()))
        }
    }
}