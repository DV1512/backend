use crate::auth::session::UserSession;
use crate::error::ServerResponseError;
use crate::extractors::{Auth, IntoSession};
use crate::generate_endpoint;
use actix_web::{Either, HttpResponse};
use tracing::info;

pub async fn delete_session(session: UserSession) -> Result<(), ServerResponseError> {
    session.delete().await?;

    info!("Session deleted successfully");

    Ok(())
}

generate_endpoint! {
    fn logout;
    method: get;
    path: "/logout";
    docs: {
        tag: "oauth",
        responses: {
            (status = 200, description = "User logged out successfully"),
            (status = 401, description = "Not logged in"),
            (status = 404, description = "User not found or invalid credentials"),
            (status = 500, description = "An error occurred when deleting the session from the database"),
        },
        security: [
            ("bearer_token" = []),
            ("cookie_session" = []),
        ]
    }
    params: {
        token: Auth
    };
    {
        let session = match token {
            Either::Left(identity) => {
                let session = identity.get_session().await;

                if session.is_none() {
                    return Err(ServerResponseError::Unauthorized);
                }

                identity.logout();
                session.unwrap()
            },
            Either::Right(session) => {
                session
            },
        };

        delete_session(session).await?;

        Ok(HttpResponse::Ok().finish())
    }
}
