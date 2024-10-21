use crate::auth::session::UserSession;
use crate::error::ServerResponseError;
use crate::generate_endpoint;
use actix_web::web;
use actix_web::HttpResponse;
use serde::Deserialize;
use tracing::{error, info};

pub async fn end_user_session(email: String) -> Result<(), ServerResponseError> {
    if let Some(session) = UserSession::fetch_by_email(email.to_string()).await {
        match session.delete().await {
            Ok(_) => {
                info!("Session deleted sucessfully");
                Ok(())
            }
            Err(err) => {
                error!("Error!");
                Err(ServerResponseError::InternalError(err.to_string()))
            }
        }
    } else {
        error!("Error!");
        return Err(ServerResponseError::NotFound);
    }
}

pub async fn logout_user(email: &str) -> Result<(), ServerResponseError> {
    end_user_session(email.to_string()).await?;
    info!("Logged out!");
    Ok(())
}

#[derive(Deserialize, Debug)]
struct LogoutRequest {
    email: String,
}

generate_endpoint! {
    fn logout_endpoint;
    method: post;
    path: "/logout";
    docs: {
        context_path: "/oauth",
        tag: "session",
        responses: {
            (status = 200, description = "User logged out successfully"),
            (status = 400, description = "Bad request"),
        }
    }
    params: {
        request: web::Query<LogoutRequest>
    };
    {
        let req_data = request.into_inner();
        match logout_user(&req_data.email).await{
            Ok(_) => {
                Ok(HttpResponse::Ok().json("User has been logged out."))
            }
            Err(err) => {
                error!("Bad Request");
                Err(ServerResponseError::BadRequest(err.to_string()))
            }
        }
    }
}
