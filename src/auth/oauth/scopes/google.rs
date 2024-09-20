use crate::auth::oauth::basic::Scopes;

#[derive(Debug, PartialEq, Clone)]
pub enum GoogleScope {
    Email,
    Profile,
    OpenId,
}

impl From<GoogleScope> for String {
    fn from(scope: GoogleScope) -> Self {
        match scope {
            GoogleScope::Email => "https://www.googleapis.com/auth/userinfo.email".to_string(),
            GoogleScope::Profile => "https://www.googleapis.com/auth/userinfo.profile".to_string(),
            GoogleScope::OpenId => "openid".to_string(),
        }
    }
}

impl From<&GoogleScope> for String {
    fn from(scope: &GoogleScope) -> Self {
        match scope {
            GoogleScope::Email => "https://www.googleapis.com/auth/userinfo.email".to_string(),
            GoogleScope::Profile => "https://www.googleapis.com/auth/userinfo.profile".to_string(),
            GoogleScope::OpenId => "openid".to_string(),
        }
    }
}

impl From<GoogleScope> for &str {
    fn from(scope: GoogleScope) -> Self {
        match scope {
            GoogleScope::Email => "https://www.googleapis.com/auth/userinfo.email",
            GoogleScope::Profile => "https://www.googleapis.com/auth/userinfo.profile",
            GoogleScope::OpenId => "openid",
        }
    }
}

impl From<&GoogleScope> for &str {
    fn from(scope: &GoogleScope) -> Self {
        match scope {
            GoogleScope::Email => "https://www.googleapis.com/auth/userinfo.email",
            GoogleScope::Profile => "https://www.googleapis.com/auth/userinfo.profile",
            GoogleScope::OpenId => "openid",
        }
    }
}

impl From<String> for GoogleScope {
    fn from(scope: String) -> Self {
        match scope.as_str() {
            "https://www.googleapis.com/auth/userinfo.email" => GoogleScope::Email,
            "https://www.googleapis.com/auth/userinfo.profile" => GoogleScope::Profile,
            "openid" => GoogleScope::OpenId,
            _ => GoogleScope::Email,
        }
    }
}

impl From<&String> for GoogleScope {
    fn from(scope: &String) -> Self {
        match scope.as_str() {
            "https://www.googleapis.com/auth/userinfo.email" => GoogleScope::Email,
            "https://www.googleapis.com/auth/userinfo.profile" => GoogleScope::Profile,
            "openid" => GoogleScope::OpenId,
            _ => GoogleScope::Email,
        }
    }
}

impl From<&str> for GoogleScope {
    fn from(scope: &str) -> Self {
        match scope {
            "https://www.googleapis.com/auth/userinfo.email" => GoogleScope::Email,
            "https://www.googleapis.com/auth/userinfo.profile" => GoogleScope::Profile,
            "openid" => GoogleScope::OpenId,
            _ => GoogleScope::Email,
        }
    }
}

impl From<&str> for &GoogleScope {
    fn from(scope: &str) -> Self {
        match scope {
            "https://www.googleapis.com/auth/userinfo.email" => &GoogleScope::Email,
            "https://www.googleapis.com/auth/userinfo.profile" => &GoogleScope::Profile,
            "openid" => &GoogleScope::OpenId,
            _ => &GoogleScope::Email,
        }
    }
}

impl From<String> for &GoogleScope {
    fn from(scope: String) -> Self {
        match scope.as_str() {
            "https://www.googleapis.com/auth/userinfo.email" => &GoogleScope::Email,
            "https://www.googleapis.com/auth/userinfo.profile" => &GoogleScope::Profile,
            "openid" => &GoogleScope::OpenId,
            _ => &GoogleScope::Email,
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct GoogleScopes {
    scopes: Vec<GoogleScope>,
}

impl From<Vec<String>> for GoogleScopes {
    fn from(scopes: Vec<String>) -> Self {
        scopes
            .iter()
            .map(|s| s.into())
            .collect::<Vec<GoogleScope>>()
            .into()
    }
}

impl From<GoogleScopes> for Vec<String> {
    fn from(scopes: GoogleScopes) -> Self {
        scopes.scopes.iter().map(|s| s.into()).collect()
    }
}

impl From<Vec<GoogleScope>> for GoogleScopes {
    fn from(scopes: Vec<GoogleScope>) -> Self {
        Self { scopes }
    }
}

impl Scopes<GoogleScope> for GoogleScopes {
    fn add_scope(mut self, scope: GoogleScope) -> Self {
        self.scopes.push(scope);
        self
    }

    fn remove_scope(mut self, scope: GoogleScope) -> Self {
        self.scopes.retain(|s| s != &scope);
        self
    }

    fn scopes(&self) -> Vec<&str> {
        self.scopes.iter().map(|s| s.into()).collect()
    }
}
