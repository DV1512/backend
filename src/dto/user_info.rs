use crate::models::user_info::{Role, UserInfo};
use crate::models::datetime::Datetime;
use crate::utils::create_dto::create_dto;

create_dto! {
    UserInfo,
    struct UserInfoDTO {
        pub email: String,
        pub url_safe_username: Option<String>,
        pub username: String,
        pub first_name: String,
        pub last_name: String,
        pub created_at: Datetime,
        pub last_login: Option<Datetime>,
        pub picture: Option<String>,
        pub role: Role,
    }
}
