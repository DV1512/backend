use crate::auth::users::get::utils::get_user_by_token;
use crate::auth::UserInfo;
use crate::utils::middleware::define_middleware;
use crate::AppState;
use actix_web::web::Data;
use actix_web::{dev::ServiceRequest, HttpMessage};
use tracing::error;

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
