use crate::__path_health_check;
use crate::models::{datetime::Datetime, thing::Thing};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(health_check),
    nest(
        (path = "/user", api = crate::auth::users::UserApi),
        (path = "/oauth", api = crate::auth::oauth::OauthApi),
    ),
    components(schemas(Datetime, Thing), responses()),
    tags(
        (name = "user", description = "User management"),
        (name = "oauth", description = "OAuth provider management"),
        (name = "auth", description = "Authentication management"),
        (name = "health", description = "Health check"),
    )
)]
pub struct DocsV1;

#[derive(OpenApi)]
#[openapi(
    nest(
        (path = "/api/v1", api = DocsV1),
    )
)]
pub struct ApiDoc;
