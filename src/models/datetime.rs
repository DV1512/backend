use serde::{Deserialize, Serialize};
use tosic_utils::utils::wrap_external::wrap_external_type;
use utoipa::openapi::{schema::ObjectBuilder, KnownFormat, RefOr, Schema, SchemaFormat, Type};
use utoipa::PartialSchema;

wrap_external_type! {
    #[derive(Debug, Serialize, Deserialize, Clone, PartialOrd, Eq, PartialEq, Default)]
    pub struct Datetime(surrealdb::sql::Datetime);
}

impl PartialSchema for Datetime {
    fn schema() -> RefOr<Schema> {
        ObjectBuilder::new()
            .schema_type(Type::String)
            .format(Some(SchemaFormat::KnownFormat(KnownFormat::DateTime)))
            .into()
    }
}

impl utoipa::ToSchema for Datetime {}
