use serde::{Deserialize, Serialize};
use std::ops::{Deref, DerefMut};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, Clone, PartialOrd, Eq, PartialEq)]
pub struct Thing(surrealdb::sql::Thing);

impl Deref for Thing {
    type Target = surrealdb::sql::Thing;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Thing {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

use utoipa::openapi::{self, schema::ObjectBuilder, SchemaFormat};

impl<'__s> utoipa::ToSchema<'__s> for Thing {
    fn schema() -> (&'__s str, openapi::RefOr<openapi::schema::Schema>) {
        (
            "Thing",
            ObjectBuilder::new()
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
                    ObjectBuilder::new().schema_type(openapi::schema::Type::String),
                )
                .into(),
        )
    }
}

impl FromIterator<Thing> for Vec<surrealdb::sql::Thing> {
    fn from_iter<T: IntoIterator<Item = Thing>>(iter: T) -> Self {
        iter.into_iter().collect()
    }
}

impl From<surrealdb::sql::Thing> for Thing {
    fn from(v: surrealdb::sql::Thing) -> Self {
        Self(v)
    }
}

impl From<Thing> for surrealdb::sql::Thing {
    fn from(v: Thing) -> Self {
        v.0
    }
}
