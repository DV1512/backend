use crate::auth::session::UserSession;
use crate::auth::UserInfo;
use crate::error::ServerResponseError;
use crate::models::refresh_token::RefreshToken;
use crate::state::AppState;
use actix_identity::Identity;
use actix_web::http::header::CacheDirective;
use actix_web::{http::header, web, HttpMessage, HttpRequest, HttpResponse};
use helper_macros::generate_endpoint;
use oauth2::AccessToken;
use rand::{
    distributions::{Alphanumeric, DistString},
    thread_rng,
};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use tracing::info;
use utoipa::openapi::path::{Parameter, ParameterBuilder, ParameterIn};
use utoipa::openapi::schema::Type;
use utoipa::openapi::{KnownFormat, Required};
use utoipa::openapi::{Object, ObjectBuilder, SchemaFormat};
use utoipa::IntoParams;
use utoipa::{ToResponse, ToSchema};

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
#[serde(tag = "grant_type", rename_all = "snake_case")]
pub(crate) enum TokenRequest {
    Password { username: String, password: String },
    RefreshToken { refresh_token: RefreshToken },
}

impl IntoParams for TokenRequest {
    fn into_params(parameter_in_provider: impl Fn() -> Option<ParameterIn>) -> Vec<Parameter> {
        let parameter_in = parameter_in_provider().unwrap_or_default();
        let grant_type_param = ParameterBuilder::new()
            .name("grant_type")
            .parameter_in(parameter_in.clone())
            .required(Required::True)
            .schema::<Object>(Some(
                ObjectBuilder::new()
                    .schema_type(Type::String)
                    .enum_values(Some(vec!["password", "refresh_token"]))
                    .build(),
            ))
            .description(Some("Type of grant being requested"))
            .build();

        let username_param = ParameterBuilder::new()
            .name("username")
            .parameter_in(parameter_in.clone())
            .required(Required::False)
            .schema::<Object>(Some(ObjectBuilder::new().schema_type(Type::String).build()))
            .description(Some("User's username"))
            .build();

        let password_param = ParameterBuilder::new()
            .name("password")
            .parameter_in(parameter_in.clone())
            .required(Required::False)
            .schema::<Object>(Some(
                ObjectBuilder::new()
                    .schema_type(Type::String)
                    .format(Some(SchemaFormat::KnownFormat(KnownFormat::Password)))
                    .build(),
            ))
            .description(Some("User's password"))
            .build();

        let refresh_token_params = ParameterBuilder::new()
            .name("refresh_token")
            .parameter_in(ParameterIn::Query)
            .required(Required::True)
            .schema::<Object>(Some(ObjectBuilder::new().schema_type(Type::String).build()))
            .description(Some("Refresh token"))
            .build();

        vec![
            grant_type_param,
            username_param,
            password_param,
            refresh_token_params,
        ]
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub(crate) struct TokenResponse<'a> {
    access_token: AccessToken,
    refresh_token: RefreshToken,
    token_type: &'a str,
    expires_in: usize,
}

#[derive(ToResponse)]
#[allow(dead_code)]
pub enum TokenResponseExample {
    #[response(examples(
            ("password" = (value = json!({
                "access_token": "ZzwuN7HvEw80MedCDOcQVRrnm3lhHBkmkpYK7TY1yDY7enjjmc",
                "refresh_token": "SMZuiT5rjv9UmfIXcYMvJQSHRRt8I8Dtg6U6o6C6SNCs80pE4o",
                "token_type": "bearer",
                "expires_in": 3600
            }), description = "Successful access token request and the credentials are returned", summary = "Successful access token request"))
    ))]
    User(#[content("application/json")] UserInfo),
}

/// Returns a random alphanumeric string of length `length`.
fn random_string(length: usize) -> String {
    Alphanumeric.sample_string(&mut thread_rng(), length)
}

impl<'a> TokenResponse<'a> {
    fn new() -> Self {
        Self {
            access_token: AccessToken::new(random_string(50)),
            refresh_token: RefreshToken::new(random_string(50)),
            token_type: "bearer",
            expires_in: 3600,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct AuthenticatedUser {
    count: usize,
    email: String,
    id: Thing,
    username: String,
}

generate_endpoint! {
    fn token;
    method: post;
    path: "/token";
    docs: {
        tag: "token",
        responses: {
            (status = 200, response = TokenResponseExample),
            (status = 404, description = "User not found or invalid credentials"),
            (status = 501, description = "Refresh token grant type is not implemented yet")
        }
    }
    params: {
        req: HttpRequest,
        state: web::Data<AppState>,
        data: web::Form<TokenRequest>,
    };
    {
        info!("Requesting access token");
        let db = state.db.clone();
        match data.0 {
            TokenRequest::RefreshToken { refresh_token: _ } => Err(ServerResponseError::NotImplementedWithMessage("Refreshing tokens not yet supported".to_string())),
            TokenRequest::Password { username, password } => {
                let db_query = format!("SELECT *, count() as count FROM user WHERE email = \"{}\" AND array::any(<-auth_for<-user_auth, |$a| !type::is::none($a.password) AND type::is::string($a.password) AND crypto::argon2::compare($a.password, \"{}\")) FETCH auth;", username, password);
                info!("DB query: {}", db_query);

                let mut res = db
                    .query(db_query)
                    .await?;

                let query_result: Option<AuthenticatedUser> = res
                    .take(0)?;

                info!("Query result: {:?}", &query_result);
                let valid_user = query_result.clone().is_some_and(|user| user.count > 0);

                if valid_user {
                    let user = query_result.clone().unwrap();
                    let user_id = user.id.clone();
                    let response = TokenResponse::new();
                    let token = response.access_token.secret().to_string();

                    let session = UserSession::new(token.clone(), Some(response.refresh_token.secret().to_string()), user.email, user_id);
                    Identity::login(&req.extensions(), token).unwrap();
                    session.create().await?;

                    Ok(HttpResponse::Ok()
                        .insert_header(header::CacheControl(vec![
                            CacheDirective::NoCache,
                            CacheDirective::NoStore,
                        ]))
                        .json(response))
                } else {
                    Err(ServerResponseError::UnauthorizedWithMessage("Invalid username or password".to_string()))
                }
            }
        }
    }
}