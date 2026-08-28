#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, newtype::FromInner, newtype::Display,
)]
pub(crate) struct NotificationObservabilityShutdownError(
    server_runtime_http::domain_types::OpentelemetrySdkObservabilityShutdownError,
);
