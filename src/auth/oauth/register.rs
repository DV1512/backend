use crate::auth::users::create::register_user;
use crate::auth::UserInfo;
use crate::state::AppState;
use actix_web::{web, HttpResponse};
use helper_macros::generate_endpoint;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use crate::dto::UserRegistrationRequest;

generate_endpoint! {
    fn register_endpoint;
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
