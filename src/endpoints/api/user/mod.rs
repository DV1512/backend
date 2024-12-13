pub mod delete;
pub mod get;
pub mod update;

use crate::endpoints::user::delete::*;
use crate::endpoints::user::get::*;
use crate::endpoints::user::update::*;
use crate::extractors::Authenticated;
use crate::models::user_info::UserInfo;
use crate::models::user_info::UserInfoExampleResponses;
use crate::models::Role;
use crate::services::user::get::{get_user_by_internal, GetUserBy};
use crate::state::AppState;
use actix_web::guard::Acceptable;
use actix_web::web;
use helper_macros::generate_endpoint;
use tracing::info;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(get_user_by, update_user, delete_user_endpoint),
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
        .service(delete_user_endpoint)
}
