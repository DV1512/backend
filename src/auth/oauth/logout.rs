use crate::auth::session::UserSession;
use crate::error::ServerResponseError;
use crate::generate_endpoint;
use actix_identity::Identity;
use actix_web::HttpResponse;
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
        tag: "session",
        responses: {
            (status = 200, description = "User logged out successfully"),
            (status = 401, description = "Not logged in"),
            (status = 404, description = "User not found or invalid credentials"),
            (status = 500, description = "An error occurred when deleting the session from the database"),
        }
    }
    params: {
        token: Identity
    };
    {
        let access_token = token.id()?;
        delete_session(access_token).await?;

        token.logout();

        Ok(HttpResponse::Ok().finish())
    }
}
