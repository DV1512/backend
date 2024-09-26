macro_rules! define_oauth_client {
    (
        $oauth_struct:ident,
        $provider_struct:ident,
        $scopes_type:ty,
        $scope_type:ty,
        $user_info_type:ty,
        {
            client_id_env: $client_id_env:expr,
            client_secret_env: $client_secret_env:expr,
            base_url_env: $base_url_env:expr,
            default_base_url: $default_base_url:expr,
            user_info_mapping: |$user_info_var:ident| $user_info_mapping:block,
        }
    ) => {
        use api_forge::ApiRequest;
        use oauth2::TokenResponse;
        use crate::auth::oauth::provider::Provider;

        #[derive(Debug, Clone)]
        pub struct $oauth_struct {
            basic: crate::auth::oauth::basic::BasicOauth<$scopes_type, $scope_type>,
            details: $provider_struct,
        }

        #[derive(Debug, Default, Clone, serde::Serialize, api_forge::Request)]
        #[request(endpoint = "", response_type = $user_info_type, authentication = Bearer)] // Adjust authentication type as needed
        struct GetUserInfoRequest;

        impl $oauth_struct {
            #[tracing::instrument]
            pub async fn new() -> Result<Self, crate::auth::oauth::error::OauthError> {
                let mut details = $provider_struct::fetch().await?;

                let config = details.get_config();

                if config.auth_url.is_none()
                    || config.token_url.is_none()
                    || config.scopes.is_none()
                    || config.user_info_url.is_none()
                {
                    return Err(crate::auth::oauth::error::OauthError::ConfigError);
                }

                let base_url = tosic_utils::prelude::env!($base_url_env, $default_base_url);
                let endpoint = config.get_redirect_endpoint();
                let redirect_url = format!("{}{}", base_url, endpoint);

                let client_id = tosic_utils::prelude::env!($client_id_env);
                let client_secret = tosic_utils::prelude::env!($client_secret_env);

                let basic = crate::auth::oauth::basic::BasicOauth::new(
                    client_id,
                    client_secret,
                    redirect_url,
                    config.get_auth_url(),
                    config.get_token_url(),
                    config.get_scopes().into(),
                )?;

                Ok(Self { basic, details })
            }

            pub fn get_auth_url(&self) -> (String, oauth2::CsrfToken) {
                self.basic.get_authorization_url()
            }

            #[tracing::instrument(skip(self, code, db))]
            async fn exchange_code_internal<T>(
                &self,
                code: String,
                db: &::std::sync::Arc<surrealdb::Surreal<T>>,
            ) -> Result<crate::auth::session::UserSession, crate::auth::oauth::error::OauthError>
            where
                T: surrealdb::Connection,
            {
                let token = self.basic.exchange_code_for_token(code).await?;

                let user_info = self.get_user_info(token.access_token().secret()).await?;

                let user_info = {
                    let $user_info_var = user_info;
                    $user_info_mapping
                };

                let user_info = user_info?;

                let user = user_info;

                let db_user = crate::auth::users::get_user(db, &user.email).await;

                let record = match db_user {
                    None => {
                        let new_user_record = crate::auth::users::create::create_user(db, user.clone()).await?;
                        crate::auth::create_auth_for_user(
                            new_user_record.clone(),
                            vec![self.details.clone().into()],
                            None,
                        )
                        .await?;
                        new_user_record
                    }
                    Some(existing_user) => {
                        if let Some(id) = existing_user.id {
                            crate::Record { id: id.into() }
                        } else {
                            unreachable!("it should be impossible to reach this since the database requires a id and will populate it");
                        }
                    }
                };

                let access_token = token.access_token().secret().to_string();
                let refresh_token = token.refresh_token().map(|t| t.secret().to_string());

                let session = match crate::auth::session::UserSession::fetch_by_email(user.email.clone()).await {
                    Some(mut s) => {
                        s.access_token = access_token;
                        s.refresh_token = refresh_token;
                        s.update().await?
                    }
                    None => {
                        crate::auth::session::UserSession::new(access_token, refresh_token, user.email, record.id)
                            .create()
                            .await?
                    }
                };

                Ok(session)
            }

            pub async fn exchange_code<C: surrealdb::Connection>(
                &self,
                code: String,
                db: &::std::sync::Arc<surrealdb::Surreal<C>>,
            ) -> Result<crate::auth::session::UserSession, crate::auth::oauth::error::OauthError> {
                self.exchange_code_internal(code, db).await
            }

            #[tracing::instrument(skip(self))]
            async fn get_user_info(&self, access_token: &str) -> Result<$user_info_type, crate::auth::oauth::error::OauthError> {
                ::tracing::debug!("Fetching user info for access token: {}", access_token);

                let user_info_url = self.details.config.clone().unwrap().get_user_info_url();

                if user_info_url.is_empty() {
                    ::tracing::error!("User info URL is empty");
                    return Err(crate::auth::oauth::error::OauthError::MissingUserInfoUrl);
                }

                let req_builder = GetUserInfoRequest::default();

                let mut headers = reqwest::header::HeaderMap::new();
                headers.insert(reqwest::header::USER_AGENT, "ThreatMapper/1.0".parse().unwrap());

                let response = req_builder
                    .send_request(&user_info_url, Some(headers), Some((access_token.to_owned(), None)))
                    .await?;

                ::tracing::debug!("User info response: {:?}", response);

                Ok(response.json::<$user_info_type>().await?)
            }
        }
    }
}

pub(crate) use define_oauth_client;
