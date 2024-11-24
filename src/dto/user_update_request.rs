use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UserUpdateRequest {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub password: Option<String>,
}
