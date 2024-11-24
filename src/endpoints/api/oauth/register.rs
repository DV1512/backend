use crate::dto::UserRegistrationRequest;
use crate::services::user::create::register_user;
use crate::state::AppState;
use actix_web::{web, HttpResponse};
use helper_macros::generate_endpoint;

generate_endpoint! {
    fn register;
    method: post;
    path: "/register";
    docs: {
        tag: "oauth",
        responses: {
            (status = 201, description = "User created successfully"),
            (status = 500, description = "An error occurred when creating the user"),
        }
    }
    params: {
        state: web::Data<AppState>,
        data: web::Json<UserRegistrationRequest>,
    };
    {
        register_user(&state.db, data.0).await?;
        Ok(HttpResponse::Created().finish())
    }
}
