use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use crate::auth::UserInfo;

#[derive(Serialize, Deserialize, ToSchema, IntoParams)]
pub struct UserRegistrationRequest {
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
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
            first_name: value.first_name.unwrap_or_default(),
            last_name: value.last_name.unwrap_or_default(),
            ..Default::default()
        }
    }
}