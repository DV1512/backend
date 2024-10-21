use actix_identity::Identity;
use crate::error::ServerResponseError;
use crate::state::AppState;
use actix_web::http::header::CacheDirective;
use actix_web::{http::header, web, HttpMessage, HttpRequest, HttpResponse, Scope};
use helper_macros::generate_endpoint;
use oauth2::{AccessToken, RefreshToken};
use rand::{
    distributions::{Alphanumeric, DistString},
    thread_rng,
};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use tracing::info;
use utoipa::{openapi, IntoParams};
use utoipa::openapi::{Discriminator, Object, ObjectBuilder, OneOf, RefOr, Schema, SchemaFormat};
use utoipa::ToSchema;
use utoipa::openapi::path::{Parameter, ParameterBuilder, ParameterIn};
use utoipa::openapi::{KnownFormat, Required};
use utoipa::openapi::schema::{Type};
use crate::auth::session::UserSession;
use crate::auth::UserInfo;
use crate::auth::users::auth::UserAuth;
use crate::models::datetime::Datetime;
use crate::Record;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
#[serde(tag = "grant_type", rename_all = "snake_case")]
pub(crate) enum TokenRequest {
    Password { username: String, password: String },
    RefreshToken { refresh_token: RefreshToken },
}

impl IntoParams for TokenRequest {
    fn into_params(
        parameter_in_provider: impl Fn() -> Option<ParameterIn>,
    ) -> Vec<Parameter> {
        let parameter_in = parameter_in_provider().unwrap_or_default();
        // Common parameter: grant_type
        let grant_type_param = ParameterBuilder::new()
            .name("grant_type")
            .parameter_in(parameter_in.clone()) // Adjust as necessary
            .required(Required::True)
            .schema::<Object>(Some(
                ObjectBuilder::new()
                    .schema_type(Type::String)
                    .enum_values(Some(vec!["password", "refresh_token"]))
                    .build(),
            ))
            .description(Some("Type of grant being requested"))
            .build();

        // Parameters for Password variant
        let password_params = vec![
            ParameterBuilder::new()
                .name("username")
                .parameter_in(parameter_in.clone()) // Adjust as necessary
                .required(Required::True)
                .schema::<Object>(Some(
                    ObjectBuilder::new()
                        .schema_type(Type::String)
                        .build(),
                ))
                .description(Some("User's username"))
                .build(),
            ParameterBuilder::new()
                .name("password")
                .parameter_in(parameter_in.clone()) // Adjust as necessary
                .required(Required::True)
                .schema::<Object>(Some(
                    ObjectBuilder::new()
                        .schema_type(Type::String)
                        .format(Some(SchemaFormat::KnownFormat(
                            KnownFormat::Password,
                        )))
                        .build(),
                ))
                .description(Some("User's password"))
                .build(),
        ];

        // Parameters for RefreshToken variant
        let refresh_token_params = vec![ParameterBuilder::new()
            .name("refresh_token")
            .parameter_in(ParameterIn::Query) // Adjust as necessary
            .required(Required::True)
            .schema::<Object>(Some(
                ObjectBuilder::new()
                    .schema_type(Type::String)
                    .build(),
            ))
            .description(Some("Refresh token"))
            .build()];

        // Combine parameters based on the variant
        let mut params = vec![grant_type_param];
        params.extend(password_params.into_iter().map(|mut param| {
            param.required = Required::False;
            param
        }));
        params.extend(refresh_token_params.into_iter().map(|mut param| {
            param.required = Required::False;
            param
        }));

        params
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
struct TokenResponse<'a> {
    access_token: AccessToken,
    refresh_token: RefreshToken,
    token_type: &'a str,
    expires_in: usize,
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
        context_path: "/oauth",
        responses: {
            (status = 200, description = "Access token request successful"),
            (status = 404, description = "User not found")
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
            TokenRequest::RefreshToken { refresh_token: _ } => Err(ServerResponseError::BadRequest(
                "Refreshing tokens not yet supported".to_string(),
            )),
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

                    let mut session = UserSession::new(response.access_token.secret().to_string(), Some(response.refresh_token.secret().to_string()), username, user_id.clone());
                    session.email = user.email;
                    let id = user_id.to_string();
                    Identity::login(&req.extensions(), id).unwrap();
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
