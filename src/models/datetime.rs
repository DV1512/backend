use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};

#[derive(Debug, Serialize, Deserialize, Clone, PartialOrd, Eq, PartialEq, Default)]
pub struct Datetime(surrealdb::sql::Datetime);

impl Deref for Datetime {
    type Target = surrealdb::sql::Datetime;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Datetime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

use utoipa::openapi::{self, schema::ObjectBuilder, SchemaFormat};

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
