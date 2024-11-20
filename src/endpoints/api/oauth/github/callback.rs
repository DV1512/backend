use actix_web::{web, HttpRequest, HttpResponse};
use tracing::{error, info};
use helper_macros::generate_endpoint;
use crate::dto::OAuthCallbackQuery;
use crate::error::ServerResponseError;
use crate::state::AppState;

generate_endpoint! {
    fn callback;
    method: get;
    path: "/callback";
    docs: {
        context_path: "/github",
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
            .github
            .exchange_code(query.code.clone(), &state.db)
            .await
        {
            Ok(session) => {
                let redirect_url = format!("{}redirect?token={}", frontend_url, session.access_token);

                Ok(HttpResponse::Found()
                    .append_header(("Location", redirect_url))
                    .finish())
            }

            Err(err) => {
                error!("Error exchanging code: {}", err);
                Err(ServerResponseError::InternalError(err.to_string()))
            }
        }
    }
}
