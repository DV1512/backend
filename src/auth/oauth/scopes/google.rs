use crate::utils::scope::define_scopes;

define_scopes!(
    GoogleScopes;
    GoogleScope {
        Email => "https://www.googleapis.com/auth/userinfo.email",
        Profile => "https://www.googleapis.com/auth/userinfo.profile",
        OpenId => "openid",
    }
);
