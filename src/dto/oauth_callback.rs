use serde::Deserialize;
use utoipa::IntoParams;

#[allow(dead_code)]
#[derive(Deserialize, Debug, IntoParams)]
pub(crate) struct OAuthCallbackQuery {
    #[param(example = "code123")]
    pub(crate) code: String,
    #[param(example = "state123")]
    pub(crate) state: String,
}