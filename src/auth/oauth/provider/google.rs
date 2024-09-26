use crate::auth::oauth::provider::OauthProviderName;
use crate::auth::oauth::scopes::google::{GoogleScope, GoogleScopes};
use serde::{Deserialize, Serialize};
use tracing::warn;
use crate::utils::provider::define_provider;

define_provider! {
    GoogleProvider;
    OauthProviderName::Google;

    auth_url: {
        env_var: "GOOGLE_AUTH_URL",
        default: "https://accounts.google.com/o/oauth2/auth"
    },
    token_url: {
        env_var: "GOOGLE_TOKEN_URL",
        default: "https://oauth2.googleapis.com/token"
    },
    user_info_url: {
        env_var: "GOOGLE_USER_INFO_URL",
        default: "https://www.googleapis.com/oauth2/v3/userinfo"
    },
    redirect_endpoint: "/api/v1/oauth/google/callback",
    scopes: GoogleScopes {
        add_scopes: [
            GoogleScope::Email,
            GoogleScope::Profile,
            GoogleScope::OpenId
        ]
    }
}