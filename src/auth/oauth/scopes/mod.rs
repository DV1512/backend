pub(crate) mod github;
pub mod google;

pub trait Scopes<T> {
    fn add_scope(self, scope: T) -> Self;

    #[allow(dead_code)]
    fn remove_scope(self, scope: T) -> Self;

    fn scopes(&self) -> Vec<&str>;
}