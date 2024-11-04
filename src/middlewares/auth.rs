use crate::utils::middleware::define_middleware;
use actix_web::{dev::ServiceRequest, HttpMessage};

#[derive(Debug, Clone)]
pub enum AuthType {
    ApiKey(String),
    AccessToken(String),
}

define_middleware! {
    /// Middleware for authenticating requests
    ///
    /// The middleware will check if the request contains an API key or an access token and authenticate the user if it exists
    ///
    /// ## Important Headers
    ///
    /// - `Authorization: Bearer <token>`
    /// - `X-Api-Key: <key>`
    pub struct AuthMiddleware {},
    pub struct AuthMiddlewareService;
    |this: &AuthMiddlewareService<S>, req: ServiceRequest| {
        let service = this.service.clone();
        let fut = async move {
            // Access headers
            let headers = req.headers();

            // Extract API key or access token
            let auth_header = headers.get("Authorization");
            let api_key_header = headers.get("X-Api-Key");

            // Placeholder for authentication result
            let mut session: Option<AuthType> = None;

            if let Some(auth_header) = auth_header {
                if let Ok(auth_str) = auth_header.to_str() {
                    if let Some(token) = auth_str.strip_prefix("Bearer ") {
                        session = Some(AuthType::AccessToken(token.into()));
                    }
                }
            } else if let Some(api_key_header) = api_key_header {
                if let Ok(api_key) = api_key_header.to_str() {
                    session = Some(AuthType::ApiKey(api_key.into()));
                }
            }

            if let Some(user) = session {
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
