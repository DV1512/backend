use serde::{Deserialize, Serialize};
use tosic_utils::wrap_external_type;
use utoipa::openapi;

wrap_external_type! {
    #[derive(Debug, Serialize, Deserialize, Clone)]
    pub struct RefreshToken(oauth2::RefreshToken);
}

impl RefreshToken {
    pub fn new(token: String) -> Self {
        Self(oauth2::RefreshToken::new(token))
    }
}

impl<'__s> utoipa::ToSchema<'__s> for RefreshToken {
    fn schema() -> (&'__s str, openapi::RefOr<openapi::schema::Schema>) {
        (
            "RefreshToken",
            openapi::schema::ObjectBuilder::new()
                .schema_type(openapi::schema::Type::String)
                .description(Some("Refresh token"))
                .into(),
        )
    }
}
