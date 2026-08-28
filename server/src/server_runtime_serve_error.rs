#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error(transparent)]
pub(crate) struct ServerRuntimeServeError(
    server_runtime_http::domain_types::ServeWithGracefulShutdownError,
);
