use crate::auth::users::get::utils::get_user_by_token;
use crate::auth::UserInfo;
use crate::AppState;
use actix_web::dev::forward_ready;
use actix_web::web::Data;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::{ok, LocalBoxFuture, Ready};
use std::sync::Arc;
use tracing::error;

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareService {
            service: Arc::new(service),
        })
    }
}

pub struct AuthMiddlewareService<S> {
    service: Arc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    #[tracing::instrument(skip(self, req), name = "AuthMiddleware")]
    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        let fut = async move {
            // Access AppState
            let app_state = req
                .app_data::<Data<AppState>>()
                .ok_or_else(|| actix_web::error::ErrorInternalServerError("AppState not found"))?;

            // Access headers
            let headers = req.headers();

            // Extract API key or access token
            let auth_header = headers.get("Authorization");
            let api_key_header = headers.get("X-Api-Key");

            // Placeholder for authentication result
            let mut authenticated_user: Option<UserInfo> = None;

            if let Some(auth_header) = auth_header {
                if let Ok(auth_str) = auth_header.to_str() {
                    if let Some(token) = auth_str.strip_prefix("Bearer ") {
                        let user = match get_user_by_token(&app_state.db, token).await {
                            Ok(user) => user,
                            Err(e) => {
                                error!("Failed to get user by token: {:?}", e);
                                return Err(actix_web::error::ErrorUnauthorized("Unauthorized"));
                            }
                        };

                        authenticated_user = Some(user);
                    }
                }
            } else if let Some(api_key_header) = api_key_header {
                if let Ok(_api_key) = api_key_header.to_str() {
                    let user = UserInfo {
                        id: None,
                        email: "".to_string(),
                        url_safe_username: "test".to_string(),
                        username: "".to_string(),
                        first_name: "".to_string(),
                        last_name: "".to_string(),
                        created_at: Default::default(),
                        last_login: None,
                        picture: None,
                        role: Default::default(),
                    };

                    authenticated_user = Some(user);
                }
            }

            if let Some(user) = authenticated_user {
                // Attach user info to request extensions
                req.extensions_mut().insert(user);

                // Proceed to the next service
                let res = service.call(req).await?;
                Ok(res)
            } else {
                // Return 401 Unauthorized
                Err(actix_web::error::ErrorUnauthorized("Unauthorized"))
            }
        };

        Box::pin(fut)
    }
}
