#[derive(
    optimal_memory_layout::OptimalMemoryLayout, Debug, thiserror::Error, newtype::FromInner,
)]
#[error("failed to shut down OpenTelemetry tracer provider: {0}")]
pub struct OpentelemetrySdkObservabilityShutdownError(opentelemetry_sdk::error::OTelSdkError);
