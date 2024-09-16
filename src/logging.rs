use tracing_subscriber::{EnvFilter, Layer, Registry, filter::LevelFilter};
use tracing_subscriber::fmt::Layer as FmtLayer;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::SubscriberExt;
use crate::server_error::ServerError;

#[inline]
/// Initialize tracing with default settings
pub fn init_tracing() -> Result<(), ServerError> {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        EnvFilter::builder()
            .with_default_directive(LevelFilter::INFO.into())
            .from_env_lossy()
    });
    let def_layer = FmtLayer::new()
        .with_line_number(true)
        .with_span_events(FmtSpan::CLOSE)
        .with_level(true)
        .with_target(true)
        .with_thread_names(true)
        .compact()
        .with_filter(filter);

    let subscriber = Registry::default().with(def_layer);

    tracing::subscriber::set_global_default(subscriber).map_err(|err| ServerError::SetGlobalDefaultError(err))?;

    Ok(())
}