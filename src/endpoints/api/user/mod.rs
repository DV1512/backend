mod get;

use crate::auth::UserInfo;
use crate::auth::Role;
use actix_web::guard::Acceptable;
use actix_web::web;
use tracing::info;
use utoipa::OpenApi;
use helper_macros::generate_endpoint;
use crate::auth::UserInfoExampleResponses;
use crate::auth::users::get::{get_user_by_internal, GetUserBy};
use crate::endpoints::user::get::*;
use crate::extractors::Authenticated;
use crate::state::AppState;

#[derive(OpenApi)]
#[openapi(
    paths(get_user_by),
    components(
        schemas(Role, UserInfo, GetUserBy),
        responses(UserInfoExampleResponses)
    )
)]
pub(crate) struct UserApi;

pub fn user_service() -> impl actix_web::dev::HttpServiceFactory {
    web::scope("/user")
        .guard(Acceptable::new(mime::APPLICATION_JSON).match_star_star())
        .service(get_user_by)
}