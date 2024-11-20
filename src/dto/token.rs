use rand::distributions::{Alphanumeric, DistString};
use rand::thread_rng;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToResponse, ToSchema};
use utoipa::openapi::path::{Parameter, ParameterBuilder, ParameterIn};
use utoipa::openapi::{KnownFormat, Object, ObjectBuilder, Required, SchemaFormat, Type};
use crate::models::access_token::AccessToken;
use crate::models::refresh_token::RefreshToken;

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
pub(crate) struct TokenResponse {
    pub(crate) access_token: AccessToken,
    pub(crate) refresh_token: RefreshToken,
    token_type: TokenType,
    expires_in: usize,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Default)]
#[serde(rename_all = "lowercase")]
pub(crate) enum TokenType {
    #[default]
    Bearer,
    Basic,
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
    Success(#[content("application/json")] TokenResponse),
}

/// Returns a random alphanumeric string of length `length`.
fn random_string(length: usize) -> String {
    Alphanumeric.sample_string(&mut thread_rng(), length)
}

impl TokenResponse {
    pub(crate) fn new() -> Self {
        Self {
            access_token: AccessToken::new(random_string(50)),
            refresh_token: RefreshToken::new(random_string(50)),
            token_type: TokenType::default(),
            expires_in: 3600,
        }
    }
}