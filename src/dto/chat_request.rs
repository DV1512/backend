use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub enum Keyword {
    Website,
    Web,
    Database,
    Backend,
    Credentials,
    Security,
    Network,
    Authentication,
    Permissions,
    Encryption
}

#[derive(Serialize, Deserialize, ToSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ChatRequest {
    Structured { prompt: String, keywords: Vec<Keyword> },
    Chat { prompt: String },
}
