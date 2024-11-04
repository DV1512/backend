use serde::{Deserialize, Serialize};
use tosic_utils::utils::wrap_external::wrap_external_type;
use utoipa::openapi::{RefOr, Schema};
use utoipa::{openapi, PartialSchema};

wrap_external_type! {
    #[derive(Debug, Serialize, Deserialize, Clone, PartialOrd, Eq, PartialEq)]
    pub(crate) struct Thing(surrealdb::sql::Thing);
}

impl PartialSchema for Thing {
    fn schema() -> RefOr<Schema> {
        openapi::schema::ObjectBuilder::new()
            .schema_type(openapi::schema::Type::Object)
            .description(Some("Thing identifier"))
            .property(
                "id",
                openapi::schema::ObjectBuilder::new().schema_type(openapi::schema::Type::String),
            )
            .required("id")
            .property(
                "tb",
                openapi::schema::ObjectBuilder::new().schema_type(openapi::schema::Type::String),
            )
            .into()
    }
}

impl utoipa::ToSchema for Thing {}
