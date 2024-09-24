mod auth;
pub(crate) mod create;
pub(crate) mod get;

use apistos::web;
use apistos::web::Scope;
pub(crate) use self::get::get_user;
use self::get::get_user_by;


pub fn user_service() -> Scope {
    web::scope("/user").service(web::resource("").route(web::get().to(get_user_by)))
}
