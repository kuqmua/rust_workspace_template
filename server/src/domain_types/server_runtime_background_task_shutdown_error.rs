#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error("{0}")]
pub(crate) struct ServerRuntimeBackgroundTaskShutdownError(
    server_runtime_http::domain_types::BackgroundTaskShutdownError,
);
