use crate::middlewares::auth::AuthType;
use crate::middlewares::logger::LogEntry;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use tracing::info;

pub fn background_logger() -> Sender<LogEntry> {
    let (log_sender, log_receiver) = mpsc::channel::<LogEntry>(100);

    tokio::spawn(async move { logger(log_receiver).await });

    log_sender
}

#[inline]
async fn logger(mut receiver: Receiver<LogEntry>) {
    while let Some(log) = receiver.recv().await {
        let method = log.method;
        let path = log.path;
        let status = log.status;
        let auth_method = log.user;

        let log = format!("Method: {}, Path: {}, Status: {}", method, path, status);

        match auth_method {
            Some(AuthType::ApiKey(key)) => {
                info!("{}, Access was granted using API key: {} to ", log, key);
            }
            Some(AuthType::AccessToken(token)) => {
                info!(
                    "{}, Access was granted using access token: {} to ",
                    log, token
                );
            }
            None => {
                info!("{}, No Auth present for this request", log);
            }
        }
    }
}