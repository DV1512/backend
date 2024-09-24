use anyhow::Result;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope, TokenUrl,
};
use std::marker::PhantomData;
use tracing::debug;

pub trait Scopes<T> {
    fn add_scope(self, scope: T) -> Self;

    #[allow(dead_code)]
    fn remove_scope(self, scope: T) -> Self;

    fn scopes(&self) -> Vec<&str>;
}

#[derive(Debug, Clone)]
pub struct BasicOauth<T, U>
where
    T: Default + Scopes<U> + Clone,
    String: From<U>,
{
    pub client: oauth2::basic::BasicClient,
    pub scopes: T,
    phantom_data: PhantomData<U>,
}

impl<T, U> BasicOauth<T, U>
where
    T: Default + Scopes<U> + Clone,
    String: From<U>,
{
    pub fn new(
        client_id: String,
        client_secret: String,
        redirect_url: String,
        auth_url: String,
        token_url: String,
        scopes: T,
    ) -> Result<Self> {
        let client_id = ClientId::new(client_id);
        let client_secret = ClientSecret::new(client_secret);
        let redirect_url = RedirectUrl::new(redirect_url)?;
        let auth_url = AuthUrl::new(auth_url)?;
        let token_url = TokenUrl::new(token_url)?;

        let client = oauth2::basic::BasicClient::new(
            client_id,
            Some(client_secret),
            auth_url,
            Some(token_url),
        )
        .set_redirect_uri(redirect_url);

        Ok(Self {
            client,
            scopes,
            phantom_data: PhantomData,
        })
    }

    fn get_scopes(&self) -> Vec<Scope> {
        let mut scopes = Vec::new();

        for scope in self.scopes.scopes() {
            scopes.push(Scope::new(scope.to_string()));
        }

        scopes
    }

    pub fn get_authorization_url(&self) -> (String, CsrfToken) {
        // TODO: Add PKCE

        // Generate the authorization URL with scopes
        let mut auth_url_builder = self.client.authorize_url(CsrfToken::new_random);

        // Add scopes to the authorization URL
        for scope in self.get_scopes() {
            auth_url_builder = auth_url_builder.add_scope(scope);
        }

        // Finalize the URL
        let (auth_url, csrf_token) = auth_url_builder.url();

        (auth_url.to_string(), csrf_token)
    }

    #[tracing::instrument(skip(self, code))]
    pub async fn exchange_code_for_token(
        &self,
        code: String,
    ) -> Result<oauth2::basic::BasicTokenResponse, anyhow::Error> {
        let code = AuthorizationCode::new(code);

        debug!("Code: {:?}", code);

        let token = self
            .client
            .exchange_code(code)
            .request_async(oauth2::reqwest::async_http_client)
            .await?;

        debug!("Token: {:?}", token);

        Ok(token)
    }
}
