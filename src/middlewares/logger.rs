use crate::auth::UserInfo;
use actix_web::dev::forward_ready;
use actix_web::http::{Method, StatusCode};
use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::{ok, LocalBoxFuture, Ready};
use tokio::sync::mpsc::Sender;
use tracing::error;

#[derive(Debug, Clone)]
pub struct LogEntry {
    pub method: Method,
    pub path: String,
    pub user: Option<UserInfo>,
    pub status: StatusCode,
}

pub struct LoggingMiddleware {
    log_sender: Sender<LogEntry>,
}

impl LoggingMiddleware {
    pub fn new(log_sender: Sender<LogEntry>) -> Self {
        Self { log_sender }
    }
}

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
        ok(LoggingMiddlewareService {
            service,
            log_sender: self.log_sender.clone(),
        })
    }
}

pub struct LoggingMiddlewareService<S> {
    service: S,
    log_sender: Sender<LogEntry>,
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

    #[tracing::instrument(skip(self, req), name = "LoggingMiddleware")]
    fn call(&self, req: ServiceRequest) -> Self::Future {
        let method = req.method().clone();
        let path = req.path().to_string();
        let user = req.extensions().get::<UserInfo>().cloned();
        let log_sender = self.log_sender.clone();

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            // Access response status code
            let status = res.status();

            // Create the log entry
            let log_entry = LogEntry {
                method,
                path,
                user,
                status,
            };

            // Send the log entry into the channel
            if let Err(e) = log_sender.send(log_entry).await {
                error!("Failed to send log entry: {}", e);
            }

            Ok(res)
        })
    }
}
