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
)]
pub struct ApiDoc;
