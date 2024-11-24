use crate::dto::OAuthCallbackQuery;
use crate::error::ServerResponseError;
use crate::state::AppState;
use actix_identity::Identity;
use actix_web::cookie::Cookie;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use helper_macros::generate_endpoint;
use tracing::{error, info};

generate_endpoint! {
    fn google_callback;
    method: get;
    path: "/callback";
    docs: {
        tag: "oauth",
        responses: {
            (status = 302, description = "Redirect to frontend"),
        }
    }
    params: {
        state: web::Data<AppState>,
        query: web::Query<OAuthCallbackQuery>,
        req: HttpRequest,
    };
    {
        info!("Google callback received");

        let oauth = state.oauth.clone();

        let frontend_url = req.url_for_static("frontend").unwrap().to_string();

        match oauth
            .google
            .exchange_code(query.code.clone(), &state.db)
            .await
        {
            Ok(session) => {
                let redirect_url = format!("{}redirect?token={}", frontend_url, session.access_token);
                Identity::login(&req.extensions(), session.access_token.clone()).unwrap();

                Ok(HttpResponse::Found()
                    .append_header(("Location", redirect_url))
                    .cookie(Cookie::new("token", session.access_token))
                    .finish())
            }

            Err(err) => {
                error!("Error exchanging code: {}", err);
                Err(ServerResponseError::InternalError(err.to_string()))
            }
        }
    }
}
