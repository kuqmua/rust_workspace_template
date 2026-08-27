#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner, newtype::Display,
)]
pub(crate) struct NotificationObservabilityInitError(
    server_runtime_http::domain_types::ObservabilityInitError,
);
