use crate::dto::PaginationResponse;
use crate::models::datetime::Datetime;
use crate::models::thing::Thing;
use serde::{Deserialize, Serialize};
use std::string::String;
use utoipa::{ToResponse, ToSchema};
#[derive(Debug, Serialize, Deserialize, PartialOrd, Eq, PartialEq, Clone, Default, ToSchema)]
pub enum Role {
    Owner,
    Admin,
    #[default]
    User,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, ToSchema, PartialOrd, Eq, PartialEq)]
pub struct UserInfo {
    #[schema(example = "user:123456")]
    pub id: Option<Thing>,
    #[schema(example = "johndoe@example.com")]
    pub email: String,
    #[schema(example = "johndoe")]
    pub url_safe_username: Option<String>,
    #[schema(example = "John Doe")]
    pub username: String,
    #[schema(example = "John")]
    pub first_name: String,
    #[schema(example = "Doe")]
    pub last_name: String,
    #[schema(example = "2021-09-15T14:28:23Z")]
    pub created_at: Datetime,
    #[schema(example = "2021-09-15T14:28:23Z")]
    pub last_login: Option<Datetime>,
    #[schema(example = "https://example.com/avatar.jpg")]
    pub picture: Option<String>,
    pub role: Role,
}

#[derive(ToResponse)]
#[allow(dead_code)]
pub enum UserInfoExampleResponses {
    #[response(examples(
        ("JohnDoe" = (value = json!({
            "id": {
                "id": "5f4d0c8f-1b78-4e3f-9d0c-0b0d0b0b0b0b",
                "tb": "user",
            },
            "email": "johndoe@example.com",
            "url_safe_username": "johndoe",
            "username": "John Doe",
            "first_name": "John",
            "last_name": "Doe",
            "created_at": "2021-09-15T14:28:23Z",
            "last_login": "2021-09-15T14:28:23Z",
            "picture": "https://example.com/avatar.jpg",
            "role": "Owner",
         }
         )))
    ))]
    User(#[content("application/json")] UserInfo),
}

#[derive(Debug, Serialize, Deserialize, PartialOrd, Eq, PartialEq, Clone)]
pub(crate) struct Users {
    pub(crate) users: Vec<UserInfo>,

    #[serde(flatten)]
    pub(crate) pagination: PaginationResponse,
}

