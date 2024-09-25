use crate::__path_health_check;
use crate::models::{datetime::Datetime, thing::Thing};
use utoipa::{Modify, OpenApi};

use utoipa::openapi::OpenApi as OpenApiSpec;

/// Constructs a new struct that implements [`Modify`] trait for [`utoipa`] documentation.
///
/// This is a not ideal way to do it, but this is the best solution I came up with.
macro_rules! version_prefix {
    ($name: ident, $version: literal) => {
        #[allow(dead_code)]
        #[doc(hidden)]
        struct $name;

        impl Modify for $name {
            fn modify(&self, openapi: &mut OpenApiSpec) {
                let paths = &openapi.paths;

                let mut new_paths = utoipa::openapi::path::Paths::new();

                for (path, path_item) in paths.paths.clone() {
                    new_paths.add_path(format!("/api/{}{}", $version, path), path_item);
                }

                openapi.paths = new_paths;
            }
        }
    };
}

version_prefix!(AddV1Prefix, "v1");

struct NormalizePath;

impl Modify for NormalizePath {
    fn modify(&self, openapi: &mut OpenApiSpec) {
        let paths = &openapi.paths;

        let mut new_paths = utoipa::openapi::path::Paths::new();

        for (path, path_item) in paths.paths.clone() {
            let new_path = path.replace("//", "/");

            new_paths.add_path(new_path, path_item);
        }

        openapi.paths = new_paths;
    }
}

#[derive(OpenApi)]
#[openapi(
    nest(
        (path = "/user", api = crate::auth::users::UserApi),
        (path = "/oauth", api = crate::auth::oauth::OauthApi),
    ),
    components(schemas(Datetime, Thing), responses()),
    tags(
        (name = "user", description = "User management"),
        (name = "oauth", description = "OAuth provider management"),
        (name = "auth", description = "Authentication management"),
    ),
    modifiers(&AddV1Prefix)
)]
pub struct DocsV1;

#[derive(OpenApi)]
#[openapi(
    paths(health_check),
    nest(
        (path = "/", api = DocsV1),
    ),
    components(schemas(Datetime, Thing), responses()),
    tags(
        (name = "health", description = "Health check")
    ),
    modifiers(&NormalizePath)
)]
pub struct ApiDocs;
