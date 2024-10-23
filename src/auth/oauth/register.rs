use crate::auth::users::create::register_user;
use crate::auth::{Role, UserInfo};
use crate::models::datetime::Datetime;
use crate::state::AppState;
use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
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

#[utoipa::path(post, path = "/register")]
#[actix_web::post("/register")]
pub async fn register_endpoint(
    state: web::Data<AppState>,
    data: web::Json<UserRegistrationRequest>,
) -> Result<impl ::actix_web::Responder, crate::error::ServerResponseError> {
    let recres = register_user(&state.db, data.0);
    Ok(HttpResponse::Created().finish())
}
