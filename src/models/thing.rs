use serde::{Deserialize, Serialize};
use tosic_utils::utils::wrap_external::wrap_external_type;
use utoipa::openapi;

wrap_external_type! {
    #[derive(Debug, Serialize, Deserialize, Clone, PartialOrd, Eq, PartialEq)]
    pub(crate) struct Thing(surrealdb::sql::Thing);
}

impl<'__s> utoipa::ToSchema<'__s> for Thing {
    fn schema() -> (&'__s str, openapi::RefOr<openapi::schema::Schema>) {
        (
            "Thing",
            openapi::schema::ObjectBuilder::new()
                .schema_type(openapi::schema::Type::Object)
                .description(Some("Thing identifier"))
                .property(
                    "id",
                    openapi::schema::ObjectBuilder::new()
                        .schema_type(openapi::schema::Type::String),
                )
                .required("id")
                .property(
                    "tb",
                    openapi::schema::ObjectBuilder::new()
                        .schema_type(openapi::schema::Type::String),
                )
                .into(),
        )
    }
}
