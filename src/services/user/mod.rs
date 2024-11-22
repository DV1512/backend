pub mod create;
pub mod get;
pub mod update;
pub mod utils;

use crate::auth::Role;
use crate::models::user_info::UserInfo;
use crate::models::user_info::UserInfoExampleResponses;
use crate::endpoints::user::get::*;
use crate::endpoints::user::update::*;
use crate::services::user::get::GetUserBy;
use actix_web::guard::Acceptable;
use actix_web::web;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(get_user_by, update_user),
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
        .service(update_user)
}
