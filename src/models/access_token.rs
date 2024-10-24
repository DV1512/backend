use serde::{Deserialize, Serialize};
use tosic_utils::wrap_external_type;
use utoipa::openapi::{ObjectBuilder, RefOr, Schema, Type};
use utoipa::PartialSchema;

wrap_external_type! {
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct AccessToken(oauth2::AccessToken);
}

impl AccessToken {
    pub fn new(token: String) -> Self {
        Self(oauth2::AccessToken::new(token))
    }
}

impl PartialSchema for AccessToken {
    fn schema() -> RefOr<Schema> {
        ObjectBuilder::new()
            .schema_type(Type::String)
            .description(Some("Access token"))
            .into()
    }
}

impl utoipa::ToSchema for AccessToken {}
