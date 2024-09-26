/// Defines a set of OAuth scopes and related types.
///
/// This macro creates an enum for individual scopes, a struct for managing multiple scopes,
/// and implements various conversion traits and methods for working with these types.
///
/// # Arguments
///
/// * `$scopes_struct` - The name of the struct to manage multiple scopes.
/// * `$scope_enum` - The name of the enum for individual scopes.
/// * `$variant` - The variants of the scope enum.
/// * `$value` - The string representation of each scope variant.
///
/// # Example
///
/// ```rust
/// define_scopes! {
///     GoogleScopes;
///     GoogleScope {
///         Email => "https://www.googleapis.com/auth/userinfo.email",
///         Profile => "https://www.googleapis.com/auth/userinfo.profile",
///         OpenId => "openid",
///     }
/// }
/// ```
///
/// This will create:
/// 1. A `GoogleScope` enum with variants `Email`, `Profile`, and `OpenId`.
/// 2. A `GoogleScopes` struct for managing multiple `GoogleScope` values.
/// 3. Implementations for converting between `GoogleScope`, `String`, and `&str`.
/// 4. Implementations for the `Scopes` trait on `GoogleScopes`.
///
/// Usage example:
///
/// ```rust
/// let scopes = GoogleScopes::default()
///     .add_scope(GoogleScope::Email)
///     .add_scope(GoogleScope::Profile);
///
/// let scope_strings: Vec<String> = scopes.scopes().into_iter().map(String::from).collect();
/// assert_eq!(scope_strings, vec![
///     "https://www.googleapis.com/auth/userinfo.email".to_string(),
///     "https://www.googleapis.com/auth/userinfo.profile".to_string(),
/// ]);
/// ```
macro_rules! define_scopes {
    (
        $scopes_struct:ident;
        $scope_enum:ident
        {
            $( $variant:ident => $value:expr ),+ $(,)?
        }
    ) => {
        #[derive(Debug, PartialEq, Clone)]
        pub enum $scope_enum {
            $( $variant ),+
        }

        impl From<$scope_enum> for String {
            fn from(scope: $scope_enum) -> Self {
                match scope {
                    $( $scope_enum::$variant => $value.to_string(), )+
                }
            }
        }

        impl From<&$scope_enum> for String {
            fn from(scope: &$scope_enum) -> Self {
                match scope {
                    $( $scope_enum::$variant => $value.to_string(), )+
                }
            }
        }

        impl From<$scope_enum> for &str {
            fn from(scope: $scope_enum) -> Self {
                match scope {
                    $( $scope_enum::$variant => $value, )+
                }
            }
        }

        impl From<&$scope_enum> for &str {
            fn from(scope: &$scope_enum) -> Self {
                match scope {
                    $( $scope_enum::$variant => $value, )+
                }
            }
        }

        impl From<String> for $scope_enum {
            fn from(scope: String) -> Self {
                match scope.as_str() {
                    $( $value => $scope_enum::$variant, )+
                    _ => panic!("Invalid scope string"),
                }
            }
        }

        impl From<&String> for $scope_enum {
            fn from(scope: &String) -> Self {
                match scope.as_str() {
                    $( $value => $scope_enum::$variant, )+
                    _ => panic!("Invalid scope string"),
                }
            }
        }

        impl From<&str> for $scope_enum {
            fn from(scope: &str) -> Self {
                match scope {
                    $( $value => $scope_enum::$variant, )+
                    _ => panic!("Invalid scope string"),
                }
            }
        }

        #[derive(Default, Debug, Clone)]
        pub struct $scopes_struct {
            scopes: Vec<$scope_enum>,
        }

        impl From<Vec<String>> for $scopes_struct {
            fn from(scopes: Vec<String>) -> Self {
                scopes
                    .iter()
                    .map(|s| s.into())
                    .collect::<Vec<$scope_enum>>()
                    .into()
            }
        }

        impl From<$scopes_struct> for Vec<String> {
            fn from(scopes: $scopes_struct) -> Self {
                scopes.scopes.iter().map(|s| s.into()).collect()
            }
        }

        impl From<Vec<$scope_enum>> for $scopes_struct {
            fn from(scopes: Vec<$scope_enum>) -> Self {
                Self { scopes }
            }
        }

        impl crate::auth::oauth::scopes::Scopes<$scope_enum> for $scopes_struct {
            fn add_scope(mut self, scope: $scope_enum) -> Self {
                self.scopes.push(scope);
                self
            }

            fn remove_scope(mut self, scope: $scope_enum) -> Self {
                self.scopes.retain(|s| s != &scope);
                self
            }

            fn scopes(&self) -> Vec<&str> {
                self.scopes.iter().map(|s| s.into()).collect()
            }
        }
    };
}

pub(crate) use define_scopes;