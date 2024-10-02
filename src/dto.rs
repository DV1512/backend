use crate::auth::{Role, UserInfo};
use crate::models::datetime::Datetime;
use crate::models::thing::Thing;
use serde::{Deserialize, Serialize};
use std::string::String;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IdDTO {
    pub id: String,
    pub tb: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserInfoDTO {
    pub id: IdDTO,
    pub email: String,
    pub url_safe_username: String,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub created_at: Datetime,
    pub last_login: Option<Datetime>,
    pub picture: Option<String>,
    pub role: Role,
}

impl From<Thing> for IdDTO {
    fn from(thing: Thing) -> Self {
        IdDTO {
            id: thing.id.to_string(),
            tb: thing.tb.clone(),
        }
    }
}

impl From<UserInfo> for UserInfoDTO {
    fn from(user: UserInfo) -> Self {
        UserInfoDTO {
            id: user.id.map_or(
                IdDTO {
                    tb: "table".to_string(),
                    id: "".to_string(),
                },
                |thing| thing.into(),
            ),
            email: user.email,
            url_safe_username: user.url_safe_username,
            username: user.username,
            first_name: user.first_name,
            last_name: user.last_name,
            created_at: user.created_at,
            last_login: user.last_login,
            picture: user.picture,
            role: user.role,
        }
    }
}
