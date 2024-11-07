use actix_web::{web, HttpResponse};
use tracing::info;
use helper_macros::generate_endpoint;
use crate::state::AppState;

generate_endpoint! {
    fn google_login;
    method: get;
    path: "/login";
    docs: {
        tag: "oauth",
        responses: {
            (status = 302, description = "Redirect to Google login page"),
        }
    }
    params: {
        state: web::Data<AppState>,
    };
    {
        info!("Redirecting to Google login page");
        let oauth = state.oauth.clone();

        let (auth_url, _csrf_token) = oauth.google.get_auth_url();

        Ok(HttpResponse::Found()
            .append_header(("Location", auth_url))
            .finish())
    }
}