use serde::{Deserialize, Serialize};
use tosic_utils::wrap_external_type;
use utoipa::openapi::{RefOr, Schema};
use utoipa::{openapi, PartialSchema};

wrap_external_type! {
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct RefreshToken(oauth2::RefreshToken);
}

impl RefreshToken {
    pub fn new(token: String) -> Self {
        Self(oauth2::RefreshToken::new(token))
    }
}

impl PartialSchema for RefreshToken {
    fn schema() -> RefOr<Schema> {
        openapi::schema::ObjectBuilder::new()
            .schema_type(openapi::schema::Type::String)
            .description(Some("Refresh token"))
            .into()
    }
}

impl utoipa::ToSchema for RefreshToken {}
