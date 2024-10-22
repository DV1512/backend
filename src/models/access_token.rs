use serde::{Deserialize, Serialize};
use tosic_utils::wrap_external_type;
use utoipa::openapi;

wrap_external_type! {
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct AccessToken(oauth2::AccessToken);
}

impl AccessToken {
    pub fn new(token: String) -> Self {
        Self(oauth2::AccessToken::new(token))
    }
}

impl<'__s> utoipa::ToSchema<'__s> for AccessToken {
    fn schema() -> (&'__s str, openapi::RefOr<openapi::schema::Schema>) {
        (
            "AccessToken",
            openapi::schema::ObjectBuilder::new()
                .schema_type(openapi::schema::Type::String)
                .description(Some("Access token"))
                .into(),
        )
    }
}
