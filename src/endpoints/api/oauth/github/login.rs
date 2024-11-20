use actix_web::{web, HttpResponse};
use tracing::info;
use helper_macros::generate_endpoint;
use crate::state::AppState;

generate_endpoint! {
    fn login;
    method: get;
    path: "/login";
    docs: {
        context_path: "/github",
        tag: "oauth",
        responses: {
            (status = 302, description = "Redirect to Github login page"),
        }
    }
    params: {
        state: web::Data<AppState>,
    };
    {
        info!("Redirecting to Github login page");
        let oauth = state.oauth.clone();

        let (auth_url, _csrf_token) = oauth.github.get_auth_url();

        Ok(HttpResponse::Found()
            .append_header(("Location", auth_url))
            .finish())
    }
}
