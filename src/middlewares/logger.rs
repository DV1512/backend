use crate::middlewares::auth::AuthType;
use crate::utils::middleware::define_middleware;
use actix_web::{dev::ServiceRequest, HttpMessage};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;
use tokio::sync::mpsc::Sender;
use tracing::error;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct LogEntry {
    pub method: String,
    pub path: String,
    pub user: Option<AuthType>,
    pub status: u16,
    pub date: Datetime,
}

define_middleware! {
    /// Middleware that logs requests and responses and sends it to a channel so that a standalone logger can consume it with minimal overhead to the request
    pub struct LoggingMiddleware {
        log_sender: Sender<LogEntry>
    },
    pub struct LoggingMiddlewareService;
    |this: &LoggingMiddlewareService<S>, req: ServiceRequest| {
        let method = req.method().clone();
        let path = req.path().to_string();
        let user = req.extensions().get::<AuthType>().cloned();
        let log_sender = this.log_sender.clone();

        let fut = this.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            // Access response status code
            let status = res.status();

            // Create the log entry
            let log_entry = LogEntry {
                method: method.to_string(),
                path,
                user,
                status: status.as_u16(),
                date: Datetime::default(),
            };

            // Send the log entry into the channel
            if let Err(e) = log_sender.send(log_entry).await {
                error!("Failed to send log entry: {}", e);
            }

            Ok(res)
        })
    }
}

impl LoggingMiddleware {
    pub fn new(log_sender: Sender<LogEntry>) -> Self {
        Self { log_sender }
    }
}
