use actix_extensible_rate_limit::backend::memory::InMemoryBackend;
use actix_extensible_rate_limit::backend::Backend;
use tracing::error;

pub struct RateLimiterData<B>
where
    B: Backend,
{
    pub backend: B,
    pub max_requests: u64,
    pub limit_duration: u64,
}

impl<B> RateLimiterData<B>
where
    B: Backend,
{
    pub(super) fn new(backend: B, max_requests: u64, limit_duration: u64) -> Self {
        Self {
            backend,
            max_requests,
            limit_duration,
        }
    }
}

impl Default for RateLimiterData<InMemoryBackend> {
    fn default() -> Self {
        let max_requests = tosic_utils::prelude::env!("LIMIT", "100")
            .parse()
            .unwrap_or_else(|e| {
                error!("Failed to parse {}: {}", "LIMIT", e);
                10
            });
        let limit_duration = tosic_utils::prelude::env!("LIMIT_DURATION", "30")
            .parse()
            .unwrap_or_else(|e| {
                error!("Failed to parse {}: {}", "LIMIT_DURATION", e);
                60
            });

        let backend = InMemoryBackend::builder().build();

        Self::new(backend, max_requests, limit_duration)
    }
}
