use crate::auth::users::create::register_user;
use crate::auth::{Role, UserInfo};
use crate::models::datetime::Datetime;
use crate::state::AppState;
use actix_web::{web, HttpResponse};
use helper_macros::generate_endpoint;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Serialize, Deserialize, ToSchema, IntoParams)]
pub struct UserRegistrationRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

impl From<UserRegistrationRequest> for UserInfo {
    fn from(value: UserRegistrationRequest) -> Self {
        Self {
            id: None,
            email: value.email,
            url_safe_username: value.username.clone(),
            username: value.username,
            first_name: String::new(),
            last_name: String::new(),
            created_at: Datetime::default(),
            last_login: None,
            picture: Some(String::new()),
            role: Role::default(),
        }
    }
}

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
