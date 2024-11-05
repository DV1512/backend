#[derive(Debug, thiserror::Error)]
pub enum OauthError {
    #[error("Config Error")]
    ConfigError,
    #[error("Missing user info URL")]
    MissingUserInfoUrl,
    #[error("Error fetching user info: {0}")]
    FetchUserInfoError(#[from] reqwest::Error),
    #[error("Error: {0}")]
    Error(#[from] anyhow::Error),
}
