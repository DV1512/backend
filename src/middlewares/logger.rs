use crate::auth::UserInfo;
use actix_web::dev::forward_ready;
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::{ok, LocalBoxFuture, Ready};

pub struct LoggingMiddleware;

impl<S, B> Transform<S, ServiceRequest> for LoggingMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Transform = LoggingMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(LoggingMiddlewareService { service })
    }
}

pub struct LoggingMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for LoggingMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Clone necessary data for logging
        let method = req.method().clone();
        let path = req.path().to_string();
        let user = req.extensions().get::<UserInfo>().cloned();

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            // Access response status code
            let status = res.status();

            // TODO: Implement database logging logic here
            // You can access:
            // - Method: method
            // - Path: path
            // - User info: user
            // - Status code: status
            // - AppState: app_state (if needed)

            // Example placeholder:
            if let Some(user) = user {
                println!(
                    "User {} accessed {} {} and received status {}",
                    user.url_safe_username, method, path, status
                );

                // TODO: Log this action into your database
            } else {
                println!(
                    "Unauthenticated access to {} {} resulted in status {}",
                    method, path, status
                );
            }

            Ok(res)
        })
    }
}
