/*
pub(crate) mod auth;
pub(crate) mod get;

use super::{Role, UserInfo, UserInfoExampleResponses};
use crate::services::user::get::{GetUserBy, __path_get_user_by};
use actix_web::guard::Acceptable;
use actix_web::web;
use utoipa::OpenApi;

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
*/
