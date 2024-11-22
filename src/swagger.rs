use crate::endpoints::__path_health;
use crate::models::{datetime::Datetime, thing::Thing};
use std::collections::BTreeMap;
use utoipa::{Modify, OpenApi};

use utoipa::openapi::security::{ApiKey, ApiKeyValue, HttpAuthScheme, HttpBuilder, SecurityScheme};
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
                let paths = openapi.paths.paths.clone();
                let mut new_paths = BTreeMap::new();

                paths.iter().for_each(|(path, item)| {
                    let new_path = &format!("/api/{}{}", $version, path);

                    new_paths.insert(new_path.clone(), item.clone());
                });

                openapi.paths.paths = new_paths;
            }
        }
    };
}

version_prefix! {
    AddV1Prefix,
    "v1"
}

struct NormalizePath;

impl Modify for NormalizePath {
    fn modify(&self, openapi: &mut OpenApiSpec) {
        let paths = openapi.paths.paths.clone();
        let mut new_paths = BTreeMap::new();

        paths.iter().for_each(|(path, item)| {
            let new_path = &path.replace("//", "/");

            new_paths.insert(new_path.clone(), item.clone());
        });

        openapi.paths.paths = new_paths;
    }
}

pub struct OpenApiSecurityConfig;

impl Modify for OpenApiSecurityConfig {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let bearer = SecurityScheme::Http(
            HttpBuilder::new()
                .scheme(HttpAuthScheme::Bearer)
                .description(Some("Bearer auth"))
                .build(),
        );
        let cookie = SecurityScheme::ApiKey(ApiKey::Cookie(ApiKeyValue::new("id")));

        if let Some(components) = &mut openapi.components {
            components.add_security_scheme("bearer_token", bearer);
            components.add_security_scheme("cookie_session", cookie);
        } else {
            openapi.components = Some(
                utoipa::openapi::ComponentsBuilder::new()
                    .security_scheme("bearer_token", bearer)
                    .security_scheme("cookie_session", cookie)
                    .build(),
            );
        }
    }
}

#[derive(OpenApi)]
#[openapi(
    nest(
        (path = "/user", api = crate::endpoints::api::user::UserApi),
        (path = "/oauth", api = crate::endpoints::api::oauth::OauthApi),
    ),
    components(schemas(Datetime, Thing), responses()),
    tags(
        (name = "user", description = "User management"),
        (name = "oauth", description = "OAuth provider management"),
    ),
    modifiers(&AddV1Prefix)
)]
pub struct DocsV1;

#[derive(OpenApi)]
#[openapi(
    paths(health),
    nest(
        (path = "/", api = DocsV1),
    ),
    components(schemas(Datetime, Thing), responses()),
    tags(
        (name = "health", description = "Health check")
    ),
    modifiers(&NormalizePath, &OpenApiSecurityConfig)
)]
pub struct ApiDocs;
