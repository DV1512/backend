use crate::auth::session::UserSession;
use crate::error::ServerResponseError;
use crate::generate_endpoint;
use actix_web::HttpResponse;
use tracing::{error, info};

pub async fn end_user_session(email: &str) -> Result<(), ServerResponseError> {
    if let Some(session) = UserSession::fetch_by_email(email.to_string()).await {
        match session.delete().await {
            Ok(_) => {
                info!("Session Deleted Sucessfully");
                println!("Success logging out");
                Ok(())
            }
            Err(err) => {
                error!("Error!");
                println!("NOT WORKING LOL");
                Err(ServerResponseError::InternalError(err.to_string()))
            }
        }
    } else {
        error!("Error!");
        println!("No such session.");
        return Err(ServerResponseError::NotFound);
    }
}

pub async fn logout_user(email: &str) -> Result<(), ServerResponseError> {
    end_user_session(email).await?;
    info!("Logged out!");
    Ok(())
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
        }
    }
    params: {
        email: String
    };
    {
        info!("Trying to logout User");
        match logout_user(&email).await{
            Ok(_) => {
            Ok(HttpResponse::Ok().json("User has been logged out."))
            }
            Err(err) => {
            error!("Internal error while logging out");
            Err(ServerResponseError::InternalError(err.to_string()))
            }
        }
    }
}
