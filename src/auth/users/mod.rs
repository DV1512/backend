mod auth;
pub(crate) mod create;
pub(crate) mod get;

pub(crate) use self::get::get_user;
use self::get::get_user_by;
use actix_web::{web, Scope};

pub fn user_service() -> Scope {
    web::scope("/user").service(get_user_by)
}
