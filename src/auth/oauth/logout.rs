use crate::auth::session::UserSession;
use crate::error::ServerResponseError;
use crate::extractors::Auth;
use crate::generate_endpoint;
use actix_web::{Either, HttpResponse};
use tracing::info;

pub async fn delete_session(token: String) -> Result<(), ServerResponseError> {
    let session = UserSession::fetch_by_access_token(token)
        .await
        .ok_or(ServerResponseError::NotFound)?;
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
        let token = match token {
            Either::Left(identity) => {
                let access_token = identity.id()?;
                identity.logout();
                access_token
            },
            Either::Right(bearer) => {
                bearer.token().to_string()
            },
        };

        delete_session(token).await?;

        Ok(HttpResponse::Ok().finish())
    }
}
