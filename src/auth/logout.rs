use crate::auth::session::UserSession;
use crate::error;
use crate::generate_endpoint;
use actix_web::HttpResponse;
use anyhow::{bail, Result};
use tracing::info;

pub async fn end_user_session(email: &str) -> Result<()> {
    if let Some(session) = UserSession::fetch_by_email(email.to_string()).await {
        session.delete().await?;
        println!("Deleted the session");
        Ok(())
    } else {
        bail!("No such session!")
    }
}

pub async fn clear_tokens(email: &str) -> Result<()> {
    if let Some(mut session) = UserSession::fetch_by_email(email.to_string()).await {
        session.access_token = String::new();
        session.refresh_token = None;
        session.update().await?;
        Ok(())
    } else {
        bail!("No such tokens");
    }
}

pub async fn logout_user(email: &str) -> Result<()> {
    clear_tokens(&email).await?;
    end_user_session(&email).await?;
    println!("Logged out!");
    Ok(())
}

generate_endpoint! {
    fn logout_user_endpoint;
    method: post;
    path: "/logout";
    docs: {
        context_path: "/src",
        tag: "session",
        responses: {
            (status = 200, description = "User logged out successfully"),
        }
    }
    params: {
        email: String,
    };
    {
        info!("Trying to logout User");
        match logout_user(&email).await{
            Ok(_)=>Ok(HttpResponse::Found()
                .append_header(("Location", "/"))
                .finish()),

            Err(e)=>{
                error!("Failed to logout User {}", e);
                Ok(HttpResponse::InternalServerError().json("Failed to logout User"))

            }
        }
    }
}
