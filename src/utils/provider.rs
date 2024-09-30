/// Defines an OAuth provider struct and implements necessary traits.
///
/// This macro creates a new struct for an OAuth provider and implements the [Provider](crate::auth::oauth::provider::Provider) trait,
/// as well as conversion methods to and from the generic [OauthProvider](crate::auth::oauth::provider::OauthProvider) struct.
///
/// # Arguments
///
/// * `$provider_struct` - The name of the provider struct to be created.
/// * `$oauth_name` - The [OauthProviderName](crate::auth::oauth::provider::OauthProviderName) variant for this provider.
/// * `auth_url` - Configuration for the authorization URL.
/// * `token_url` - Configuration for the token URL.
/// * `user_info_url` - Configuration for the user info URL.
/// * `redirect_endpoint` - The redirect endpoint for this provider.
/// * `scopes` - The scope type and default scopes for this provider.
/// * `additional_config` - Optional additional configuration key-value pairs.
///
/// # Example
///
/// ```rust
/// define_provider! {
///     GoogleProvider;
///     OauthProviderName::Google;
///
///     auth_url: {
///         env_var: "GOOGLE_AUTH_URL",
///         default: "https://accounts.google.com/o/oauth2/auth"
///     },
///     token_url: {
///         env_var: "GOOGLE_TOKEN_URL",
///         default: "https://oauth2.googleapis.com/token"
///     },
///     user_info_url: {
///         env_var: "GOOGLE_USER_INFO_URL",
///         default: "https://www.googleapis.com/oauth2/v3/userinfo"
///     },
///     redirect_endpoint: "/api/v1/oauth/google/callback",
///     scopes: GoogleScopes {
///         add_scopes: [
///             GoogleScope::Email,
///             GoogleScope::Profile,
///             GoogleScope::OpenId
///         ]
///     }
/// }
/// ```
///
/// This will create a `GoogleProvider` struct and implement the necessary traits and methods.
macro_rules! define_provider {
    (
        $provider_struct:ident;
        $oauth_name:expr;

        auth_url: {
            env_var: $auth_url_env_var:expr,
            default: $default_auth_url:expr
        },
        token_url: {
            env_var: $token_url_env_var:expr,
            default: $default_token_url:expr
        },
        user_info_url: {
            env_var: $user_info_url_env_var:expr,
            default: $default_user_info_url:expr
        },
        redirect_endpoint: $redirect_endpoint:expr,
        scopes: $scopes_type:ty {
            add_scopes: [ $( $scope:expr ),* $(,)? ]
        }
        $(,
        additional_config: {
            $( $config_key:expr => $config_value:expr ),* $(,)?
        }
        )?
    ) => {
        #[derive(Debug, Clone, Serialize, Deserialize, Ord, PartialOrd, Eq, PartialEq, Hash)]
        pub(crate) struct $provider_struct {
            pub(crate) id: surrealdb::sql::Thing,
            pub(crate) name: crate::auth::oauth::provider::OauthProviderName,
            pub(crate) config: Option<crate::auth::oauth::provider::OauthProviderConfig>,
        }

        impl From<crate::auth::oauth::provider::OauthProvider> for $provider_struct {
            fn from(v: crate::auth::oauth::provider::OauthProvider) -> Self {
                Self {
                    id: v.id,
                    name: v.name,
                    config: v.config,
                }
            }
        }

        impl From<$provider_struct> for crate::auth::oauth::provider::OauthProvider {
            fn from(v: $provider_struct) -> Self {
                Self {
                    id: v.id,
                    name: v.name,
                    config: v.config,
                }
            }
        }

        impl crate::auth::oauth::provider::Provider for $provider_struct {
            const NAME: crate::auth::oauth::provider::OauthProviderName = $oauth_name;

            fn get_config(&mut self) -> crate::auth::oauth::provider::OauthProviderConfig {
                if let Some(config) = &self.config {
                    config.clone()
                } else {
                    use std::collections::BTreeMap;
                    use tracing::warn;
                    use crate::auth::oauth::provider::OauthProviderConfig;
                    use crate::auth::oauth::scopes::Scopes;

                    warn!(
                        "Provider config not found for name: {:?}. Please add it to the database.",
                        Self::NAME
                    );

                    let auth_url = tosic_utils::prelude::env!($auth_url_env_var, $default_auth_url);
                    let token_url = tosic_utils::prelude::env!($token_url_env_var, $default_token_url);
                    let user_info_url = tosic_utils::prelude::env!($user_info_url_env_var, $default_user_info_url);

                    let scopes = <$scopes_type>::default()
                        $( .add_scope($scope) )*;

                    let redirect_endpoint = $redirect_endpoint.to_string();

                    let additional_config = BTreeMap::new();
                    $(
                        $(
                            additional_config.insert($config_key.to_string(), $config_value.to_string());
                        )*
                    )?

                    let config = OauthProviderConfig {
                        auth_url: Some(auth_url),
                        token_url: Some(token_url),
                        scopes: Some(scopes.into()),
                        user_info_url: Some(user_info_url),
                        redirect_endpoint: Some(redirect_endpoint),
                        additional_config,
                    };

                    self.config = Some(config.clone());
                    config
                }
            }
        }
    }
}
pub(crate) use define_provider;
