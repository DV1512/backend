use crate::auth::oauth::provider::OauthProviderName;
use crate::auth::oauth::scopes::github::{GithubScope, GithubScopes};
use crate::utils::provider::define_provider;
use serde::{Deserialize, Serialize};
use tracing::warn;

define_provider! {
    GithubProvider;
    OauthProviderName::Github;

    auth_url: {
        env_var: "GITHUB_AUTH_URL",
        default: "https://github.com/login/oauth/authorize"
    },
    token_url: {
        env_var: "GITHUB_TOKEN_URL",
        default: "https://github.com/login/oauth/access_token"
    },
    user_info_url: {
        env_var: "GITHUB_USER_INFO_URL",
        default: "https://api.github.com/user"
    },
    redirect_endpoint: "/api/v1/oauth/github/callback",
    scopes: GithubScopes {
        add_scopes: [
            GithubScope::ReadUser,
            GithubScope::UserEmail
        ]
    }
}
