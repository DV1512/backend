use crate::models::user_info::UserInfo;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserCreationRequest {
    pub username: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserRegistrationRequest {
    #[serde(flatten)]
    pub user: UserCreationRequest,
    pub password: String,
}

impl From<UserRegistrationRequest> for UserInfo {
    fn from(value: UserRegistrationRequest) -> Self {
        Self {
            id: None,
            email: value.user.email,
            url_safe_username: None,
            username: value.user.username,
            first_name: value.user.first_name.unwrap_or_default(),
            last_name: value.user.last_name.unwrap_or_default(),
            ..Default::default()
        }
    }
}
