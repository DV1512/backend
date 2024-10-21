pub(crate) mod auth;
pub(crate) mod create;
pub(crate) mod get;

pub(crate) use self::get::get_user;
use self::get::get_user_by;
use super::{Role, UserInfo, UserInfoExampleResponses};
use crate::auth::users::get::__path_get_user_by;
use actix_web::{web, Scope};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(get_user_by),
    components(schemas(Role, UserInfo), responses(UserInfoExampleResponses))
)]
pub(crate) struct UserApi;

pub fn user_service() -> Scope {
    web::scope("/user").service(get_user_by)
}
