#[derive(
    proc_macro_optimal_memory_layout::OptimalMemoryLayout,
    Debug,
    thiserror::Error,
    proc_macro_newtype_from_inner::FromInner,
)]
#[error("failed to shut down OpenTelemetry tracer provider: {0}")]
pub struct OpentelemetrySdkObservabilityShutdownError(opentelemetry_sdk::error::OTelSdkError);
