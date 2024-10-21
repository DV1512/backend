use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "snake_case")]
pub(crate) enum GrantType {
    Password,
    RefreshToken,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct AccessTokenRequestDTO {
    pub grant_type: GrantType,
    pub username: String,
    pub password: String,
    pub refresh_token: Option<String>,
    pub scope: Option<String>,
}
