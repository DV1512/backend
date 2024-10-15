use actix_web::{web, HttpResponse, Scope};
use helper_macros::generate_endpoint;

generate_endpoint! {
    fn local_login;
    method: get;
    path: "/health";
    docs: {
        tag: "health",
        context_path: "/",
        responses: {
            (status = 200, description = "Everything works just fine!")
        }
    }
    {
        Ok(HttpResponse::Ok().body("Everything works just fine!"))
    }
}

pub fn local_oauth_service() -> Scope {
    web::scope("/local").service(local_login)
}
