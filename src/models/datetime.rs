use serde::{Deserialize, Serialize};
use tosic_utils::utils::wrap_external::wrap_external_type;
use utoipa::openapi::{self, schema::ObjectBuilder, SchemaFormat};

wrap_external_type! {
    #[derive(Debug, Serialize, Deserialize, Clone, PartialOrd, Eq, PartialEq, Default)]
    pub struct Datetime(surrealdb::sql::Datetime);
}

impl<'__s> utoipa::ToSchema<'__s> for Datetime {
    fn schema() -> (&'__s str, openapi::RefOr<openapi::schema::Schema>) {
        (
            "Datetime",
            ObjectBuilder::new()
                .schema_type(openapi::schema::Type::String)
                .format(Some(SchemaFormat::KnownFormat(
                    openapi::KnownFormat::DateTime,
                )))
                .into(),
        )
    }
}
